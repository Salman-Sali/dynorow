use quote::quote;

use crate::{AsExpr, StructInfo};

pub fn generate_has_pk_value_template(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    if !struct_info.is_generated_pk_value() {
        return quote! {};
    }

    let struct_name = struct_info.struct_name.as_expr();
    let pk_value = struct_info.pk_value.clone().unwrap();
    quote! {
        impl dynorow::traits::has_pk_value_template::HasPkValueTemplate for #struct_name {
            fn get_pk_value_template() -> String {
                #pk_value.into()
            }
        }
    }
}
