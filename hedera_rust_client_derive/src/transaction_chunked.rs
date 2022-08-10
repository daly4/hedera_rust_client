use darling::{FromDeriveInput};
use quote::{quote, ToTokens};

use crate::utils::{ExeFields, ServiceFields, map_exe};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(hedera_rust_client_derive), supports(struct_any))]
pub struct TransactionChunked {
    ident: syn::Ident,
    service: ServiceFields,
    #[darling(default)]
    exe: Option<ExeFields>,
}

impl ToTokens for TransactionChunked {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let TransactionChunked {
            ref ident,
            ref service,
            ref exe,
        } = &*self;

        let method_service_name = service.method_service_name();
        let method_service_fn = service.method_service_fn();

        let (
            get_node_account_id,
            get_method,
            make_request,
            advance_request,
            map_response_status,
            should_retry,
            map_response,
        ) = map_exe(
            "crate::transaction::transaction_get_node_account_id",
            "Self::get_method",
            "crate::transaction::transaction_make_request",
            "crate::transaction::transaction_advance_request",
            "crate::transaction::transaction_map_response_status",
            "crate::transaction::transaction_should_retry",
            "crate::transaction::transaction_map_response",
            exe,
        );

        tokens.extend(quote! {
            impl #ident {
                fn get_method(_request: &crate::executor::Request, channel: &mut crate::channel::Channel) -> std::result::Result<crate::executor::Method, crate::error::HederaError> {
                    let mut service = channel.#method_service_name();
                    let call: Box<dyn FnOnce(crate::proto::services::Transaction) -> crate::executor::TransactionResponseType + Send + Sync + 'static> = Box::new(move |pb_tx| {
                        Box::pin(async move {
                            service.#method_service_fn(pb_tx).await
                        })
                    });
                    return Ok(crate::executor::Method::Transaction(call)); 
                }
    
                fn on_freeze(&mut self) -> std::result::Result<(), crate::error::HederaError> {
                    self.transaction.set_transaction_body_data(Some(self.services.proto()?));
                    Ok(())
                }
    
                pub async fn freeze_with(&mut self, client: Option<&crate::client::Client>) -> std::result::Result<&mut Self, crate::error::HederaError> {
                    if self.is_frozen() {
                        return Ok(self);
                    }
                    self.transaction.init_fee(client)?;
                    self.transaction.init_transaction_id(client)?;

                    if let Some(cli) = client {
                        if cli.auto_validate_checksums() {
                            self.validate_network_on_ids(cli)?;
                        }
                    }
    
                    let data_len = self.data.len();
                    let required_chunks = ((data_len + (CHUNK_SIZE - 1)) / CHUNK_SIZE);
    
                    if required_chunks > self.max_chunks {
                        return Err(crate::error::HederaError::MaxChunksExceeded(required_chunks, self.max_chunks));
                    }
    
                    let initial_transaction_id = self.transaction.transaction_id()?;
                    let mut next_transaction_id = initial_transaction_id.clone();
                    let initial_transaction_id = initial_transaction_id.to_proto()?;
                    
                    self.transaction.clear_transactions();
    
                    let mut start_index = 0usize;
                    let mut end_index = 0usize;
    
                    for i in 0..required_chunks {
                        start_index = i * CHUNK_SIZE;
                        end_index = start_index + CHUNK_SIZE;
    
                        if end_index > data_len {
                            end_index = data_len;
                        }
    
                        self.transaction.add_transaction_id(next_transaction_id.clone())?;
                        self.transaction.set_transaction_body_transaction_id(next_transaction_id.clone());
    
                        // prepare inner services data
                        self.on_freeze_chunk(
                            Some(initial_transaction_id.clone()),
                            start_index,
                            end_index,
                            i,
                            required_chunks
                        )?;
    
                        // set services inner data
                        self.on_freeze()?;
    
                        for node_account_id in self.transaction.node_account_ids().into_iter() {
                            self.transaction.freeze_with_account_id(node_account_id)?;
                        }
    
                        if let Some(start) = next_transaction_id.transaction_valid_start {
                            next_transaction_id.transaction_valid_start = Some(start + chrono::Duration::nanoseconds(1));
                        }
                    }
                    Ok(self)
                }
    
                async fn execute_all(&mut self, client: &crate::client::Client) -> std::result::Result<Vec<crate::transaction_response::TransactionResponse>, crate::error::HederaError> {
                    if !self.is_frozen() {
                        self.freeze_with(Some(client)).await?;
                    }
    
                    let tx_id = self.transaction_id()?;
            
                    if let Some(tx_account_id) = tx_id.account_id {
                        if client.operator_account_id() == tx_account_id {
                            self.transaction.sign_with_operator(client).await?;
                        }
                    }
    
                    let size = (self.transaction.signed_transactions.len() / self.transaction.node_account_ids_len());
                    let mut list: Vec<crate::transaction_response::TransactionResponse> = Vec::with_capacity(size);
    
                    for i in 0..size {
                        let res = crate::executor::execute(
                            crate::executor::Request::Transaction(self.transaction.clone()),
                            client,
                            #get_node_account_id,
                            #get_method,
                            #make_request,
                            #advance_request,
                            #map_response_status,
                            #should_retry,
                            #map_response
                        ).await?;
                        list[i] = res.to_transaction()?;
                    }
                    Ok(list)
                }
                
                pub async fn execute_async(&mut self, client: &crate::client::Client) -> std::result::Result<crate::transaction_response::TransactionResponse, crate::error::HederaError> {
                    let list = self.execute_all(client).await?;
                    if list.is_empty() {
                        return Err(crate::error::HederaError::NoResultTransactions);
                    }
                    Ok(list[0].clone())
                }
            }
        });
    }     
}