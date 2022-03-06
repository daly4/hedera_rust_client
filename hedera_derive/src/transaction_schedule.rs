use proc_macro2::TokenStream;
use quote::{quote};
use syn::{
    DeriveInput, 
};

pub fn gen_transaction_schedule(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    quote! {
        impl #name {
            pub fn schedule(&mut self) -> std::result::Result<crate::schedule_create_transaction::ScheduleCreateTransaction, crate::error::HederaError> {
                self.require_not_frozen()?;
                self.on_freeze()?;
                let scheduled = self.transaction.scheduled()?;
                Ok(scheduled)
            }
        }
    }
}