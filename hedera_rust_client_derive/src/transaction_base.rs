use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn gen_transaction_base(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // Build the output, possibly using quasi-quotation
    quote! {
        impl #name {
            pub async fn add_signature(
                &mut self,
                public_key: crate::crypto::PublicKey,
                signature: crate::crypto::Signature
            ) -> std::result::Result<&mut Self, crate::error::HederaError> {
                if !self.is_frozen() {
                    self.freeze().await?;
                }
                self.transaction.add_signature(public_key, signature)?;
                Ok(self)
            }

            pub fn signatures(&self) -> std::result::Result<std::collections::HashMap<crate::account_id::AccountId, std::collections::HashMap<crate::crypto::PublicKey, Vec<u8>>>, crate::error::HederaError> {
                self.transaction.signatures()
            }

            pub fn sign(&mut self, private_key: &crate::crypto::PrivateKey) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.transaction.sign(private_key)?;
                Ok(self)
            }

            pub fn sign_with<F: Fn(&Vec<u8>) -> crate::crypto::Signature>(
                &mut self,
                public_key: &crate::crypto::PublicKey,
                signer: F
            ) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.transaction.sign_with(public_key, signer)?;
                Ok(self)
            }

            pub async fn sign_with_operator(&mut self, client: &crate::client::Client) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.transaction.sign_with_operator(client).await?;
                Ok(self)
            }

            fn require_one_node_account_id(&self) -> std::result::Result<(), crate::error::HederaError> {
                self.transaction.require_one_node_account_id()
            }

            fn require_not_frozen(&self) -> std::result::Result<(), crate::error::HederaError> {
                if self.is_frozen() {
                    return Err(crate::error::HederaError::TransactionImmutable);
                }
                Ok(())
            }

            pub fn is_frozen(&self) -> bool {
                self.transaction.is_frozen()
            }

            pub async fn freeze(&mut self) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.freeze_with(None).await
            }

            // Common
            pub fn to_bytes(&mut self) -> std::result::Result<Vec<u8>, crate::error::HederaError> {
                self.transaction.to_bytes()
            }

            pub fn transaction_hash(&mut self) -> std::result::Result<Vec<u8>, crate::error::HederaError> {
                self.transaction.transaction_hash()
            }

            pub fn max_transaction_fee(&self) -> crate::hbar::Hbar {
                self.transaction.max_transaction_fee()
            }

            // sets the max transaction fee for this Transaction.
            pub fn set_max_transaction_fee(&mut self, fee: crate::hbar::Hbar) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.transaction.set_max_transaction_fee(fee)?;
                Ok(self)
            }

            pub fn transaction_memo(&self) -> String {
                self.transaction.transaction_memo()
            }

            // sets the memo for this Transaction.
            pub fn set_transaction_memo(&mut self, memo: String) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.transaction.set_transaction_memo(memo)?;
                Ok(self)
            }

            pub fn transaction_valid_duration(&self) -> Option<chrono::Duration> {
                self.transaction.transaction_valid_duration()
            }

            pub fn set_transaction_valid_duration(
                &mut self,
                duration: Option<chrono::Duration>
            ) -> std::result::Result<&mut Self, crate::error::HederaError>{
                self.transaction.set_transaction_valid_duration(duration)?;
                Ok(self)
            }

            pub fn transaction_id(&self) -> std::result::Result<crate::transaction_id::TransactionId, crate::error::HederaError> {
                self.transaction.transaction_id()
            }

            pub fn set_transaction_id(&mut self, transaction_id: crate::transaction_id::TransactionId) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.transaction.set_transaction_id(transaction_id)?;
                Ok(self)
            }

            pub fn node_account_ids(&self) -> Vec<crate::account_id::AccountId> {
                self.transaction.node_account_ids()
            }

            pub fn set_node_account_ids(&mut self, node_ids: Vec<crate::account_id::AccountId>) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.transaction.set_node_account_ids(node_ids)?;
                Ok(self)
            }

            pub fn set_max_retry(&mut self, max_retry: u8) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.transaction.max_retry = max_retry;
                Ok(self)
            }
            pub fn set_min_backoff(&mut self, backoff: u64) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.transaction.min_backoff = Some(backoff);
                Ok(self)
            }
            pub fn set_max_backoff(&mut self, backoff: u64) -> std::result::Result<&mut Self, crate::error::HederaError> {
                self.transaction.max_backoff = Some(backoff);
                Ok(self)
            }
        }
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.transaction.to_string())
            }
        }
        impl std::default::Default for #name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}
