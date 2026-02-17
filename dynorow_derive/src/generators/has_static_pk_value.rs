use quote::quote;

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_has_pk_value_token(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    if !struct_info.is_static_pk_value() {
        return quote! {};
    }

    let struct_name = struct_info.struct_name.as_expr();
    let pk_value = struct_info.pk_value.as_ref().unwrap();
    quote! {
        impl dynorow::traits::has_pk_value::HasStaticPkValue for #struct_name {
            fn get_static_pk_value() -> String {
                String::from(#pk_value)
            }
        }
    }
}
