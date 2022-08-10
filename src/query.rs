use crate::client::Client;
use crate::error::HederaError;
use crate::executor::{ProtoRequest, Request, Response};

use crate::proto::{services, ToProto};
use crate::query_header::QueryHeader;
use crate::response_type::ResponseType;
use crate::signed_transaction::SignedTransaction;
use crate::status::Status;
use crate::transaction_body::TransactionBody;
use crate::transaction_id::TransactionId;
use crate::AccountId;
use crate::Hbar;

pub fn query_make_request_header(
    query: &Query,
    query_header: &mut QueryHeader,
) -> Result<(), HederaError> {
    if query.is_payment_required && !query.payment_transactions.is_empty() {
        query_header.payment =
            Some(query.payment_transactions[query.next_payment_transaction_index].clone())
    }
    query_header.response_type = ResponseType::AnswerOnly;
    Ok(())
}

pub fn cost_query_make_request_header(
    query_header: &mut QueryHeader,
    client: &Client,
) -> Result<(), HederaError> {
    let payment_transaction = query_make_payment_transaction(None, None, client, None)?;
    query_header.payment = Some(payment_transaction);
    query_header.response_type = ResponseType::CostAnswer;
    Ok(())
}

pub fn query_make_request(request: &mut Request) -> Result<ProtoRequest, HederaError> {
    let query = request.get_query_mut()?;
    Ok(ProtoRequest::Query(query.services.clone()))
}

#[allow(dead_code)]
pub fn query_advance_request(request: &mut Request) -> Result<(), HederaError> {
    let query = request.get_query_mut()?;
    if query.is_payment_required && !query.payment_transactions.is_empty() {
        query.next_payment_transaction_index =
            (query.next_payment_transaction_index + 1) % query.payment_transactions.len();
    }
    Ok(())
}

pub fn cost_query_advance_request(request: &mut Request) -> Result<(), HederaError> {
    let query = request.get_query_mut()?;
    query.next_payment_transaction_index =
        (query.next_payment_transaction_index + 1) % query.node_account_ids.len();
    Ok(())
}

pub fn query_should_retry(status: &Status, _response: &Response) -> bool {
    *status == Status::Busy
}

#[allow(dead_code)]
pub fn query_get_node_account_id(request: &Request) -> Result<AccountId, HederaError> {
    let query = request.get_query()?;
    if !query.node_account_ids.is_empty() {
        let account_id: AccountId =
            query.node_account_ids[query.next_payment_transaction_index].clone();
        return Ok(account_id);
    }
    return Err(HederaError::ValueNotSet("node AccountId's".to_string()));
}

pub fn cost_query_get_node_account_id(request: &Request) -> Result<AccountId, HederaError> {
    let query = request.get_query()?;
    let account_id: AccountId =
        query.node_account_ids[query.next_payment_transaction_index].clone();
    Ok(account_id)
}

pub fn query_make_payment_transaction(
    transaction_id: Option<TransactionId>,
    node_account_id: Option<AccountId>,
    client: &Client,
    cost: Option<Hbar>,
) -> Result<services::Transaction, HederaError> {
    let mut account_amounts = Vec::new();
    if let Some(node_account_id) = &node_account_id {
        let cost = match cost {
            Some(cost) => cost,
            None => return Err(HederaError::ValueNotSet("cost".to_string())),
        };
        let cost = cost.as_tinybar();
        account_amounts.push(services::AccountAmount {
            account_id: Some(node_account_id.to_proto()?),
            amount: cost,
            is_approval: false,
        });
        account_amounts.push(services::AccountAmount {
            account_id: Some(client.operator_account_id().to_proto()?),
            amount: -cost,
            is_approval: false,
        });
    }
    let mut body = TransactionBody::new();
    body.transaction_id = transaction_id;
    body.node_account_id = node_account_id;
    body.transaction_fee = Hbar::new(1.0);
    body.data = Some(services::transaction_body::Data::CryptoTransfer(
        services::CryptoTransferTransactionBody {
            transfers: Some(services::TransferList { account_amounts }),
            token_transfers: Vec::new(),
        },
    ));

    let body_bytes = body.to_proto_bytes()?;
    let signature = client.sign_with_operator(&body_bytes);
    let mut signed_transaction = SignedTransaction::with_body_bytes(body_bytes);
    signed_transaction.add_signature_pair(client.to_signature_pair_protobuf(&signature))?;
    let signed_transaction_bytes = signed_transaction.to_proto_bytes()?;

    #[allow(deprecated)]
    Ok(services::Transaction {
        body: None,
        sigs: None,
        body_bytes: Vec::new(),
        sig_map: None,
        signed_transaction_bytes,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub services: services::Query,

    pub payment_transaction_id: Option<TransactionId>,
    pub payment_transactions: Vec<services::Transaction>,
    pub node_account_ids: Vec<AccountId>,

    pub query_payment: Hbar,
    pub max_query_payment: Hbar,
    pub next_payment_transaction_index: usize,
    pub next_transaction_index: usize,
    pub max_retry: u8,
    pub min_backoff: Option<u64>,
    pub max_backoff: Option<u64>,

    pub is_payment_required: bool,
}

impl Query {
    pub fn new(is_payment_required: bool) -> Query {
        Query {
            services: services::Query { query: None },
            payment_transaction_id: None,
            payment_transactions: Vec::new(),
            node_account_ids: Vec::new(),
            query_payment: Hbar::zero(),
            max_query_payment: Hbar::zero(),
            next_payment_transaction_index: 0,
            next_transaction_index: 0,
            max_retry: 5,
            min_backoff: None,
            max_backoff: None,
            is_payment_required,
        }
    }

    pub fn with_max_query_payment(payment: Hbar) -> Query {
        let mut query = Query::new(true);
        query.max_query_payment = payment;
        query
    }

    pub fn generate_payments_for_node_account_ids(
        &mut self,
        client: &Client,
        cost: Hbar,
    ) -> Result<(), HederaError> {
        let transaction_id = match &self.payment_transaction_id {
            Some(v) => v,
            None => {
                return Err(HederaError::ValueNotSet(
                    "payment_transaction_id".to_string(),
                ))
            }
        };
        for node_id in self.node_account_ids.iter() {
            let transaction = query_make_payment_transaction(
                Some(transaction_id.clone()),
                Some(node_id.clone()),
                &client,
                Some(cost),
            )?;
            self.payment_transactions.push(transaction);
        }
        Ok(())
    }
}
