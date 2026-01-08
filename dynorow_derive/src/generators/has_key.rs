use quote::quote;

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_has_key_token(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name = struct_info.struct_name.as_expr();
    let pk_key = struct_info.get_pk_key();

    let token = match struct_info.get_sk_key() {
        Some(sk_key) => {
            quote! {
                dynorow::key::Key::CompositeKey {
                    partition_key: String::from(#pk_key),
                    sort_key: String::from(#sk_key)
                }
            }
        },
        None => {
            quote! {
                dynorow::key::Key::PartitionKey {
                    key: String::from(#pk_key)
                }
            }
        },
    };

    quote! {
        impl dynorow::traits::has_key::HasKey for #struct_name {
            fn get_key() -> dynorow::key::Key {
                #token          
            }
        }
    }
}