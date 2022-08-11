use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn gen_query_base(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // Build the output, possibly using quasi-quotation
    quote! {
        impl #name {
            pub fn set_max_query_payment(&mut self, max_payment: crate::hbar::Hbar) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.query.max_query_payment = max_payment;
                Ok(self)
            }
            pub fn set_query_payment(&mut self, payment: crate::hbar::Hbar) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.query.query_payment = payment;
                Ok(self)
            }
            pub fn set_node_account_ids(&mut self, node_account_ids: Vec<crate::account_id::AccountId>) -> std::result::Result<&mut Self, crate::error::HederaError> {
                let mut ids = node_account_ids.clone();
                self.query.node_account_ids.append(&mut ids);
                Ok(self)
            }
            pub fn set_max_retry(&mut self, max_retry: u8) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.query.max_retry = max_retry;
                Ok(self)
            }
            pub fn set_min_backoff(&mut self, backoff: u64) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.query.min_backoff = Some(backoff);
                Ok(self)
            }
            pub fn set_max_backoff(&mut self, backoff: u64) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.query.max_backoff = Some(backoff);
                Ok(self)
            }
        }
        impl std::default::Default for #name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}
