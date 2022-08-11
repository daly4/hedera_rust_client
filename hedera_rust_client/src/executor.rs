use num::pow;
use std::cmp::min;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tracing::{debug, info, instrument, trace};

use crate::channel::Channel;
use crate::client::Client;
use crate::error::HederaError;
use crate::proto::services;
use crate::query::Query;
use crate::query_response::QueryResponse;
use crate::status::Status;
use crate::transaction::Transaction;
use crate::transaction_response::TransactionResponse;
use crate::AccountId;

pub type QueryResponseType = Pin<
    Box<
        dyn Future<Output = Result<tonic::Response<services::Response>, tonic::Status>>
            + Send
            + 'static,
    >,
>;
pub type TransactionResponseType = Pin<
    Box<
        dyn Future<Output = Result<tonic::Response<services::TransactionResponse>, tonic::Status>>
            + Send
            + 'static,
    >,
>;

pub enum Method {
    Query(Box<dyn FnOnce(services::Query) -> QueryResponseType + Send + Sync + 'static>),
    Transaction(
        Box<dyn FnOnce(services::Transaction) -> TransactionResponseType + Send + Sync + 'static>,
    ),
}

#[derive(Debug)]
pub enum Request {
    Query(Query),
    Transaction(Transaction),
}

impl Request {
    pub fn get_query(&self) -> Result<&Query, HederaError> {
        match *self {
            Request::Query(ref query) => Ok(query),
            _ => Err(HederaError::QueryRequestTypeError),
        }
    }

    pub fn get_transaction(&self) -> Result<&Transaction, HederaError> {
        match *self {
            Request::Transaction(ref tx) => Ok(tx),
            _ => Err(HederaError::TransactionRequestTypeError),
        }
    }

    pub fn get_query_mut(&mut self) -> Result<&mut Query, HederaError> {
        match *self {
            Request::Query(ref mut query) => Ok(query),
            _ => Err(HederaError::QueryRequestTypeError),
        }
    }

    pub fn get_transaction_mut(&mut self) -> Result<&mut Transaction, HederaError> {
        match *self {
            Request::Transaction(ref mut tx) => Ok(tx),
            _ => Err(HederaError::TransactionRequestTypeError),
        }
    }
}

#[derive(Debug)]
pub enum ProtoRequest {
    Query(services::Query),
    Transaction(services::Transaction),
}

impl ProtoRequest {
    pub fn get_query(&self) -> Result<&services::Query, HederaError> {
        match *self {
            ProtoRequest::Query(ref query) => Ok(query),
            _ => Err(HederaError::QueryRequestTypeError),
        }
    }

    pub fn get_transaction(&self) -> Result<&services::Transaction, HederaError> {
        match *self {
            ProtoRequest::Transaction(ref tx) => Ok(tx),
            _ => Err(HederaError::TransactionRequestTypeError),
        }
    }
}

#[derive(Debug)]
pub enum Response {
    Query(services::Response),
    Transaction(services::TransactionResponse),
}

impl Response {
    pub fn get_proto_query(&self) -> Result<&services::response::Response, HederaError> {
        match *self {
            Response::Query(ref r) => {
                if let Some(ref response) = r.response {
                    Ok(response)
                } else {
                    return Err(HederaError::NoResponse);
                }
            }
            _ => Err(HederaError::NoQueryResponse),
        }
    }

    pub fn get_transaction(&self) -> Result<&services::TransactionResponse, HederaError> {
        match *self {
            Response::Transaction(ref v) => Ok(v),
            _ => Err(HederaError::NoTransactionResponse),
        }
    }
}

#[derive(Debug)]
pub enum IntermediateResponse {
    Query(QueryResponse),
    Transaction(TransactionResponse),
}

impl IntermediateResponse {
    pub fn to_query(self) -> Result<QueryResponse, HederaError> {
        match self {
            IntermediateResponse::Query(v) => Ok(v),
            _ => Err(HederaError::NoQueryResponse),
        }
    }

    pub fn to_proto_query(self) -> Result<services::response::Response, HederaError> {
        match self {
            IntermediateResponse::Query(r) => {
                if let Some(services) = r.services {
                    Ok(services)
                } else {
                    return Err(HederaError::NoResponse);
                }
            }
            _ => Err(HederaError::NoQueryResponse),
        }
    }

    pub fn to_transaction(self) -> Result<TransactionResponse, HederaError> {
        match self {
            IntermediateResponse::Transaction(v) => Ok(v),
            _ => Err(HederaError::NoTransactionResponse),
        }
    }
}

#[instrument(skip_all)]
pub async fn execute(
    mut request: Request,
    client: &Client,
    get_node_account_id: fn(&Request) -> Result<AccountId, HederaError>,
    get_method: fn(&Request, &mut Channel) -> Result<Method, HederaError>,
    make_request: fn(&mut Request) -> Result<ProtoRequest, HederaError>,
    advance_request: fn(&mut Request) -> Result<(), HederaError>,
    map_response_status: fn(&Request, &Response) -> Result<Status, HederaError>,
    should_retry: fn(&Status, &Response) -> bool,
    map_response: fn(
        Request,
        Response,
        AccountId,
        ProtoRequest,
    ) -> Result<IntermediateResponse, HederaError>,
) -> Result<IntermediateResponse, HederaError> {
    // get type
    let max_attempts = match client.max_attempts() {
        Some(max) => max,
        None => match &request {
            Request::Query(query) => query.max_retry,
            Request::Transaction(transaction) => transaction.max_retry,
        },
    };

    let mut min_backoff = client.min_backoff();
    let mut max_backoff = client.max_backoff();
    match &request {
        Request::Query(query) => {
            if let Some(backoff) = query.min_backoff {
                min_backoff = backoff;
            }
            if let Some(backoff) = query.max_backoff {
                max_backoff = backoff;
            }
        }
        Request::Transaction(transaction) => {
            if let Some(backoff) = transaction.min_backoff {
                min_backoff = backoff;
            }
            if let Some(backoff) = transaction.max_backoff {
                max_backoff = backoff;
            }
        }
    }

    trace!(
        "max_attempts: {}, min_backoff: {}, max_backoff: {}",
        max_attempts,
        min_backoff,
        max_backoff
    );
    for attempt in 1..max_attempts {
        debug!("attempt {}/{}", attempt, max_attempts);
        let node_account_id = get_node_account_id(&request)?;

        let node = client.node_for_account_id(&node_account_id).await?;

        let mut node_lock = node.write().await;
        node_lock.in_use();

        if !node_lock.is_healthy() {
            let wait = node_lock.wait();
            drop(node_lock);
            info!("node {} was not healthy", node_account_id);
            tokio::time::sleep(wait).await;
        } else {
            drop(node_lock);
        }

        let mut node_write = node.write().await;
        let channel = node_write.node_channel();
        drop(node_write);

        let mut channel = channel?;

        let method = get_method(&request, &mut channel);
        let method = match method {
            Ok(v) => v,
            Err(_) => {
                continue;
            }
        };

        let proto_request = make_request(&mut request)?;
        advance_request(&mut request)?;

        let proto_response = match method {
            Method::Query(func) => {
                let proto_query = proto_request.get_query()?;
                match func(proto_query.clone()).await {
                    Ok(v) => Ok(Response::Query(v.into_inner())),
                    Err(status) => Err(status),
                }
            }
            Method::Transaction(func) => {
                let proto_tx = proto_request.get_transaction()?;
                match func(proto_tx.clone()).await {
                    Ok(v) => Ok(Response::Transaction(v.into_inner())),
                    Err(status) => Err(status),
                }
            }
        };

        let response = match proto_response {
            Ok(v) => v,
            Err(status) => {
                let code = status.code();
                if code != tonic::Code::Ok
                    && (code == tonic::Code::Unavailable || code == tonic::Code::ResourceExhausted)
                {
                    let mut node_write = node.write().await;
                    node_write.increase_delay();
                    drop(node_write);
                    continue;
                }
                return Err(HederaError::ProtoClientFailed(code));
            }
        };

        let mut node_write = node.write().await;
        node_write.decrease_delay();
        let account_id = node_write.account_id()?;
        drop(node_write);

        let response_status = map_response_status(&request, &response)?;

        if should_retry(&response_status, &response) && attempt <= max_attempts {
            debug!("will retry on attempt {}/{}", attempt, max_attempts);
            delay_for_attempt(min_backoff, max_backoff, attempt).await;
            continue;
        }

        if response_status != Status::Ok && response_status != Status::Success {
            return Err(HederaError::FailedPreCheck(response_status));
        }

        return map_response(request, response, account_id, proto_request);
    }
    return Err(HederaError::MaxAttempsExceeded(max_attempts));
}

pub async fn delay_for_attempt(min_backoff: u64, max_backoff: u64, attempt: u8) {
    let delay = min(min_backoff * pow(u64::from(attempt), 2), max_backoff);
    tokio::time::sleep(Duration::from_millis(delay)).await;
}
