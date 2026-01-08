use quote::quote;

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_has_pk_value_token(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let Some(pk_value) = &struct_info.pk_value else {
        return quote! {};
    };

    let struct_name = struct_info.struct_name.as_expr();

    quote! {
        impl dynorow::traits::has_pk_value::HasPkValue for #struct_name {
            fn get_pk_value() -> String {
                String::from(#pk_value)
            }
        }
    }
}