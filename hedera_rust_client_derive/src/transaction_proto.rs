use darling::{ast, util, FromDeriveInput, FromField};
use quote::{quote, ToTokens};

use crate::utils::ProtoTypeFields;

#[derive(Debug, FromField)]
#[darling(attributes(hedera_rust_client_derive))]
pub struct FromStructField {
    ident: Option<syn::Ident>,
    #[darling(default)]
    to_proto: bool,
    #[darling(default)]
    to_option_proto: bool,
    #[darling(default)]
    to_proto_vec: Option<syn::Ident>,
    #[darling(default)]
    to_proto_with_fn: Option<syn::Path>,
    #[darling(default)]
    rename: Option<syn::Ident>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(hedera_rust_client_derive),
    forward_attrs(hedera_rust_client_derive)
)]
pub struct TransactionProto {
    ident: syn::Ident,
    data: ast::Data<util::Ignored, FromStructField>,
    proto: ProtoTypeFields,
}

impl ToTokens for TransactionProto {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let TransactionProto {
            ref ident,
            ref data,
            ref proto,
        } = &*self;

        let proto_type = proto.proto_type();
        let proto_enum = proto.proto_enum();

        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        // Generate the actual values to fill the fromat string.
        let field_list = fields
            .into_iter()
            .enumerate()
            .map(|(i, f)| {
                let field_ident = if let Some(field) = &f.rename {
                    quote!(#field)
                } else {
                    f.ident
                        .as_ref()
                        .map(|v| quote!(#v))
                        .unwrap_or_else(|| {
                            let i = syn::Index::from(i);
                            quote!(#i)
                        })
                };
                if f.to_proto {
                    quote!(#field_ident: self.#field_ident.to_proto()?)
                } else if f.to_option_proto {
                    quote!(#field_ident: self.#field_ident.as_ref().map(|x| x.to_proto()).transpose()?)
                } else if f.to_proto_vec.is_some() {
                    let ty = f.to_proto_vec.clone().unwrap();
                    quote!(#field_ident: self.#field_ident.iter().map(|x| x.to_proto()).collect::<std::result::Result<Vec<crate::proto::services::#ty>,crate::error::HederaError>>()?)
                } else if f.to_proto_with_fn.is_some() {
                    let with_fn = f.to_proto_with_fn.clone().unwrap();
                    quote!(#field_ident: #with_fn(&self.#field_ident)?)
                } else {
                    quote!(#field_ident: self.#field_ident.clone())
                }
            })
            .collect::<Vec<_>>();

        tokens.extend(quote! {
            impl #ident {
                pub fn proto(&self) -> std::result::Result<crate::proto::services::transaction_body::Data, crate::error::HederaError> {
                    use crate::proto::ToProto;
                    let data = crate::proto::services::transaction_body::Data::#proto_enum(crate::proto::services::#proto_type {
                        #(#field_list),*
                    });
                    Ok(data)
                }
            }
        });
    }
}
