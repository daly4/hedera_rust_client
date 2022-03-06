#![allow(dead_code)]

use darling::{FromDeriveInput};
use quote::{quote, ToTokens};

use crate::utils::{ProtoFields, ServiceFields, ExeFields, map_exe};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(hedera_derive), supports(struct_any))]
pub struct QueryExecute {
    ident: syn::Ident,
    #[darling(default)]
    exe: Option<ExeFields>,
    #[darling(default)]
    proto: Option<ProtoFields>,
    #[darling(default)]
    service: Option<ServiceFields>,

}

impl ToTokens for QueryExecute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let QueryExecute {
            ref ident,
            ref exe,
            proto: _, 
            service: _,
        } = &*self;

        let (
            get_node_account_id,
            get_method,
            make_request,
            advance_request,
            map_response_status,
            should_retry,
            map_response,
        ) = map_exe(
            "crate::query::cost_query_get_node_account_id",
            "Self::get_method",
            "crate::query::query_make_request",
            "crate::query::cost_query_advance_request",
            "Self::map_response_status",
            "crate::query::query_should_retry",
            "Self::map_response",
            exe,
        );

        tokens.extend(quote! {
            impl #ident {
                pub async fn execute_async(&mut self, client: &crate::client::Client) -> std::result::Result<crate::executor::IntermediateResponse, crate::error::HederaError> {
                    if self.query.node_account_ids.is_empty() {
                        self.query.node_account_ids = client.node_account_ids_for_execute().await;
                    }
                    crate::query::query_make_request_header(&self.query, &mut self.header)?;
                    self.build_query()?;
                
                    let res = crate::executor::execute(
                        crate::executor::Request::Query(self.query.clone()),
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
