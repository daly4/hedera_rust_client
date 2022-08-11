use darling::FromDeriveInput;
use quote::{quote, ToTokens};

use crate::utils::{map_exe, ExeFields, ServiceFields};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(hedera_rust_client_derive), supports(struct_any))]
pub struct TransactionExecutable {
    ident: syn::Ident,
    service: ServiceFields,
    #[darling(default)]
    exe: Option<ExeFields>,
}

impl ToTokens for TransactionExecutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let TransactionExecutable {
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
    
                pub async fn freeze_with(&mut self, client: Option<&crate::client::Client>) -> std::result::Result<&mut Self, crate::error::HederaError> {
                    if self.is_frozen() {
                        return Ok(self);
                    }
                    self.transaction.init_fee(client)?;
                    self.transaction.init_transaction_id(client)?;
                    self.on_freeze()?;
                    if let Some(cli) = client {
                        if cli.auto_validate_checksums() {
                            self.validate_network_on_ids(cli)?;
                        }
                    }
                    self.transaction.freeze_with(client).await?;
                    Ok(self)
                }
    
                fn on_freeze(&mut self) -> std::result::Result<(), crate::error::HederaError> {
                    self.transaction.set_transaction_body_data(Some(self.services.proto()?));
                    Ok(())
                }
                
                pub async fn execute_async(&mut self, client: &crate::client::Client) -> std::result::Result<crate::executor::IntermediateResponse, crate::error::HederaError> {
                    if !self.is_frozen() {
                        self.freeze_with(Some(client)).await?;
                    }
            
                    let tx_id = self.transaction_id()?;
            
                    if let Some(tx_account_id) = tx_id.account_id {
                        if client.operator_account_id() == tx_account_id {
                            self.transaction.sign_with_operator(client).await?;
                        }
                    }
                    
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
                    Ok(res)
                }
            }
        });
    }
}
