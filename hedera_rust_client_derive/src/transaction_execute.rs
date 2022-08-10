use proc_macro2::TokenStream;
use quote::{quote};
use syn::{
    DeriveInput,
};

pub fn gen_transaction_execute(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // Build the output, possibly using quasi-quotation
    quote! {
        impl #name {
            pub async fn execute(&mut self, client: &crate::client::Client) -> std::result::Result<crate::transaction_response::TransactionResponse, crate::error::HederaError> {
                let res = self.execute_async(client).await?;
                let tx_response = res.to_transaction()?;
                Ok(tx_response)
            }
        }
    }
}