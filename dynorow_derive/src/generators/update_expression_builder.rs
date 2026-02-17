use crate::{
    AsExpr, StructInfo,
    struct_info::{field_info::FieldInfo, field_type::FieldType},
};

use quote::{ToTokens, format_ident, quote};

pub fn generate_update_expression_builder_token(
    struct_info: &StructInfo,
) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();
    let struct_name = struct_info.struct_name.clone();
    let expression_builder_struct_name_expr =
        format!("{}UpdateExpressionBuilder", struct_name.clone()).as_expr();

    let mut field_tokens = quote! {};
    for field in &struct_info.get_handled_fields() {
        if matches!(field.field_type, FieldType::Map(_)) && !field.is_serde {
            generate_dynomap_field_function_token(&field).to_tokens(&mut field_tokens);
        } else {
            generate_field_function_token(&field).to_tokens(&mut field_tokens);
        }
    }

    quote! {
        impl #struct_name_expr {
            pub fn update_expression_builder() -> #expression_builder_struct_name_expr {
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

pub fn generate_field_function_token(field: &FieldInfo) -> proc_macro2::TokenStream {
    let key = field.get_key_str();
    let function_name = field.name.to_string().as_expr();
    let field_type_token = field.get_type_token();
    quote! {
        pub fn #function_name(self) -> dynorow::UpdateExpressionBuilder<#field_type_token> {
            dynorow::UpdateExpressionBuilder::<#field_type_token>::new(#key.into())
        }
    }
}

pub fn generate_dynomap_field_function_token(field: &FieldInfo) -> proc_macro2::TokenStream {
    let key = field.get_key_str();
    let function_name = field.name.to_string().as_expr();
    let fields_function_name = format!("{}_fields", field.name).to_string().as_expr();
    let field_type_token = field.field_syn_type.clone();

    let x = format_ident!(
        "{}DynoMapUpdateExpressionBuilder",
        field.field_type.to_string().replace(" ", "")
    );

    quote! {
        pub fn #function_name(self) -> dynorow::UpdateExpressionBuilder<#field_type_token> {
            dynorow::UpdateExpressionBuilder::<#field_type_token>::new(#key.into())
        }

        pub fn #fields_function_name(self) -> #x  {
            #field_type_token::dynomap_update_expression_builder(#key)
        }
    }
}
