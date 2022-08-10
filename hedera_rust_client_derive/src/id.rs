use darling::{FromDeriveInput};
use quote::{quote, ToTokens};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(hedera_rust_client_derive))]
pub struct Id {
    ident: syn::Ident,
    field_name: syn::Ident,
}

impl ToTokens for Id {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Id {
            ref ident,
            ref field_name,
        } = &*self;

        let field_name = field_name.clone();

        tokens.extend(quote! {
            impl #ident {
                pub fn new(shard_num: i64, realm_num: i64, #field_name: i64, checksum: Option<crate::id::IdChecksum>) -> Self {
                    #ident {
                        shard_num,
                        realm_num,
                        #field_name,
                        checksum,
                    }
                }
    
                pub fn simple(#field_name: i64) -> Self {
                    Self::new(0, 0, #field_name, None)
                }
    
                pub fn from_solidity_address(s: &str) -> std::result::Result<Self, crate::error::HederaError> {
                    let (shard_num, realm_num, num) = crate::entity_id::id_from_solidity_address(s)?;
                    Ok(Self::new(shard_num, realm_num, num, None))
                }
    
                pub fn is_zero(&self) -> bool {
                    self.shard_num == 0 && self.realm_num == 0 && self.#field_name == 0
                }
    
                pub fn to_string_with_checksum(&self, client:&crate::client::Client) -> std::result::Result<String, crate::error::HederaError> {
                    let cs = client.ledger_id().for_checksum();
                    let check = crate::entity_id::checksum(&cs, &crate::entity_id::format_id(&self.shard_num,&self.realm_num,&self.#field_name))?;
                    let s = crate::entity_id::format_id_with_checksum(&self.shard_num,&self.realm_num,&self.#field_name,&check);
                    Ok(s) 
                }
    
                pub fn to_solidity_address(&self) -> std::result::Result<String, crate::error::HederaError>  {
                    crate::entity_id::id_to_solidity_address(self.shard_num, self.realm_num, self.#field_name)
                }
            }
    
            impl std::fmt::Display for #ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let s = if let Some(check) = &self.checksum {
                        crate::entity_id::format_id_with_checksum(&self.shard_num, &self.realm_num, &self.#field_name, check)
                    } else {
                        crate::entity_id::format_id(&self.shard_num, &self.realm_num, &self.#field_name)
                    };
                    write!(f, "{}", s)
                }
            }
    
            impl std::str::FromStr for #ident {
                type Err = crate::error::HederaError;
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    let (shard_num, realm_num, num, checksum, _) = crate::id::id_from_string(s)?;
                    Ok(#ident::new(shard_num, realm_num, num, checksum))
                }
            }
        });
    }     
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(hedera_rust_client_derive))]
pub struct IdProto {
    ident: syn::Ident,
    field_name: syn::Ident,
}

impl ToTokens for IdProto {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let IdProto {
            ref ident,
            ref field_name,
        } = &*self;

        let field_name = field_name.clone();

        tokens.extend(quote! {
            impl From<crate::proto::services::#ident> for #ident {
                fn from(services: crate::proto::services::#ident) -> #ident {
                    #ident {
                        shard_num: services.shard_num,
                        realm_num: services.realm_num,
                        #field_name: services.#field_name,
                        checksum: None,
                    }
                }
            }
    
            impl crate::proto::ToProto<crate::proto::services::#ident> for #ident {
                fn to_proto(&self) -> std::result::Result<crate::proto::services::#ident, crate::error::HederaError> {
                    Ok(crate::proto::services::#ident {
                        shard_num: self.shard_num,
                        realm_num: self.realm_num,
                        #field_name: self.#field_name
                    })
                }
            }
        });
    }     
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(hedera_rust_client_derive))]
pub struct IdSup {
    ident: syn::Ident,
    field_name: syn::Ident,
}

impl ToTokens for IdSup {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let IdSup {
            ref ident,
            ref field_name,
        } = &*self;

        let field_name = field_name.clone();

        tokens.extend(quote! {
            impl std::cmp::PartialEq for #ident  {
                fn eq(&self, other: &Self) -> bool {
                    (&self.shard_num, &self.realm_num, &self.#field_name) == (&other.shard_num, &other.realm_num, &other.#field_name)
                }
            }
    
            impl std::cmp::Eq for #ident {}
    
            impl Ord for #ident {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    (&self.shard_num, &self.realm_num, &self.#field_name).cmp(&(&other.shard_num, &other.realm_num, &other.#field_name))
                }
            }
            
            impl PartialOrd for #ident {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
        });
    }     
}


#[derive(Debug, FromDeriveInput)]
#[darling(attributes(hedera_rust_client_derive))]
pub struct IdValidateChecksum {
    ident: syn::Ident,
    field_name: syn::Ident,
}

impl ToTokens for IdValidateChecksum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let IdValidateChecksum {
            ref ident,
            ref field_name,
        } = &*self;

        let field_name = field_name.clone();

        tokens.extend(quote! {
            impl crate::entity_id::ValidateChecksum for #ident {
                fn validate_checksum(&self, client:&crate::client::Client) -> std::result::Result<(), crate::error::HederaError> {
                    crate::entity_id::validate(
                        client,
                        &self.shard_num,
                        &self.realm_num,
                        &self.#field_name,
                        &self.checksum,
                    )
                }
            }
        });
    }     
}