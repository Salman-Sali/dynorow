use crate::{AsExpr, StructInfo, struct_info::field_info::FieldInfo};
use quote::{ToTokens, quote};

pub fn generate_dynomap_update_expression_builder_token(
    struct_info: &StructInfo,
) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();
    let expression_builder_struct_name_expr =
        format!("{}DynoMapUpdateExpressionBuilder", struct_info.struct_name).as_expr();

    let mut field_tokens = quote! {};
    for field in &struct_info.get_handled_fields() {
        generate_field_function_token(&field).to_tokens(&mut field_tokens);
    }

    quote! {
        impl #struct_name_expr {
            pub fn dynomap_update_expression_builder(parent_name: &str) -> #expression_builder_struct_name_expr {
                #expression_builder_struct_name_expr {
                    parent_name: parent_name.into()
                }
            }
        }

        pub struct #expression_builder_struct_name_expr {
            pub parent_name: String
        }

        impl #expression_builder_struct_name_expr {
            #field_tokens
        }
    }
}

pub fn generate_field_function_token(field: &FieldInfo) -> proc_macro2::TokenStream {
    let key = field.get_key_str();
    let function_name = field.name.to_string().as_expr();
    let field_type_token = field.get_type_token();
    quote! {
        pub fn #function_name(self) -> dynorow::UpdateExpressionBuilder<#field_type_token> {
            dynorow::UpdateExpressionBuilder::<#field_type_token>::new(
                &format!("{}.{}", self.parent_name, #key))
        }
    }
}
