use quote::{quote, ToTokens};

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_as_projection(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name = struct_info.struct_name.as_expr();
    let projection = struct_info.generate_projection_expression();

    let mut as_projection_names_expr = quote! {};
    for field in struct_info.get_handled_fields() {
        let field_name = field.get_key_str();
        let field_variable_name = field.as_projection_variable();
        quote! {result.insert(#field_variable_name.into(), #field_name.into());}.to_tokens(&mut as_projection_names_expr);
    }

    quote::quote! {
        impl dynorow::traits::as_projection::AsProjection for #struct_name {
            fn as_projection() -> String {
                String::from(#projection)
            }

            fn as_projection_names() -> std::collections::HashMap<String, String> {
                let mut result: std::collections::HashMap<String, String> = std::collections::HashMap::new();
                #as_projection_names_expr
                result
            }
        }
    }
}