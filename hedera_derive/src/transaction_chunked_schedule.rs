use proc_macro2::TokenStream;
use quote::{quote};
use syn::{
    DeriveInput, 
};

pub fn gen_transaction_chunked_schedule(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    quote! {
        impl #name {
            pub fn schedule(&mut self) -> std::result::Result<crate::schedule_create_transaction::ScheduleCreateTransaction, crate::error::HederaError> {
                self.require_not_frozen()?;

                let data_len = self.data.len();
                let required_chunks = ((data_len + (CHUNK_SIZE - 1)) / CHUNK_SIZE);

                if required_chunks > 1 {
                    return Err(crate::error::HederaError::MaxChunksExceeded(required_chunks, 1));
                }

                self.on_freeze_chunk(None, 0, data_len, 1, 1)?;

                let scheduled = self.transaction.scheduled()?;
                Ok(scheduled)
            }
        }
    }
}
