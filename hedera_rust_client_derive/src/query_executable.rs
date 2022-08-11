#![allow(dead_code)]

use darling::FromDeriveInput;
use quote::{quote, ToTokens};

use crate::utils::{ExeFields, ProtoFields, ServiceFields};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(hedera_rust_client_derive), supports(struct_any))]
pub struct QueryExecutable {
    ident: syn::Ident,
    proto: ProtoFields,
    service: ServiceFields,
    #[darling(default)]
    exe: Option<ExeFields>,
}

impl ToTokens for QueryExecutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let QueryExecutable {
            ref ident,
            ref proto,
            ref service,
            exe: _,
        } = &*self;

        let proto_enum = proto.proto_enum();
        let response_enum = proto.response_enum();
        let method_service_name = service.method_service_name();
        let method_service_fn = service.method_service_fn();

        tokens.extend(quote! {
            impl #ident {
                fn build_query(&mut self) -> std::result::Result<(), crate::error::HederaError> {
                    self.services.header = Some(self.header.to_proto()?); // set header
                    self.query.services.query = Some(crate::proto::services::query::Query::#proto_enum(self.services.clone()));
                    Ok(())
                }
    
                fn get_method(_request: &crate::executor::Request, channel: &mut crate::channel::Channel) -> std::result::Result<crate::executor::Method, crate::error::HederaError> {
                    let mut service = channel.#method_service_name();
                    let call: Box<dyn FnOnce(services::Query) -> crate::executor::QueryResponseType + Send + Sync + 'static> = Box::new(move |pb_tx| {
                        Box::pin(async move {
                            service.#method_service_fn(pb_tx).await
                        })
                    });
                    return Ok(crate::executor::Method::Query(call)); 
                }
    
                fn map_response_helper(response: &crate::executor::Response) -> std::result::Result<crate::query_response::QueryResponse, crate::error::HederaError> {
                    let r = response.get_proto_query()?;
                    if let crate::proto::services::response::Response::#response_enum(ref res) = r {
                        if let Some(h) = &res.header {
                            let header = crate::response_header::ResponseHeader::try_from(h.clone())?;
                            return Ok(crate::query_response::QueryResponse::new(header, Some(r.clone())));
                        }
                        return Err(crate::error::HederaError::NoResponseHeader);
                    }
                    Err(crate::error::HederaError::UnexpectedProtoResponseType(format!("{:?}", r)))
                }
                
                fn map_response_status(
                    _request: &crate::executor::Request, 
                    response: &crate::executor::Response
                ) -> std::result::Result<crate::status::Status, crate::error::HederaError> {
                    let query_response = Self::map_response_helper(response)?;
                    Ok(query_response.header.status)
                }
        
                fn map_response(
                    _request: crate::executor::Request, 
                    response: crate::executor::Response, 
                    _account_id: crate::account_id::AccountId, 
                    _proto_request: crate::executor::ProtoRequest
                ) -> std::result::Result<crate::executor::IntermediateResponse, crate::error::HederaError> {
                    let query_response = Self::map_response_helper(&response)?;
                    Ok(crate::executor::IntermediateResponse::Query(query_response))
                }
            }
        });
    }
}
