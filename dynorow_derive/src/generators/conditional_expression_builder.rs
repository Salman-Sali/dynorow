use quote::{ToTokens, quote};

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_conditional_expression_builder_token(
    struct_info: &StructInfo,
) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();
    let expression_builder_struct_name_expr =
        format!("{}ConditionalExpressionBuilder", struct_info.struct_name).as_expr();

    let mut field_tokens = quote! {};
    for field in &struct_info.get_handled_fields() {
        generate_field_function_token(&field.name, &field.get_key_str())
            .to_tokens(&mut field_tokens);
    }

    let pk_key = struct_info.get_pk_key();
    if struct_info
        .find_in_handled_fields("partition_key")
        .is_none()
    {
        generate_field_function_token("partition_key", &pk_key).to_tokens(&mut field_tokens);
    }

    if let Some(sk_key) = struct_info.get_sk_key() {
        if struct_info.find_in_handled_fields("sort_key").is_none() {
            generate_field_function_token("sort_key", &sk_key).to_tokens(&mut field_tokens);
        }
    }

    quote! {
        impl #struct_name_expr {
            pub fn conditional_expression_builder() -> #expression_builder_struct_name_expr {
                #expression_builder_struct_name_expr {}
            }
        }

        pub struct #expression_builder_struct_name_expr {
        }

        impl #expression_builder_struct_name_expr {
            #field_tokens
        }
    }
}

pub fn generate_field_function_token(field_name: &str, key: &str) -> proc_macro2::TokenStream {
    let function_name = field_name.to_string().as_expr();
    quote! {
        pub fn #function_name(self) -> dynorow::ConditionalExpressionBuilder {
            dynorow::ConditionalExpressionBuilder::new(#key.into())
        }
    }
}
