use quote::{ToTokens, quote};

use crate::{
    struct_info::{StructInfo, field_info::FieldInfo},
    utils::as_expr::AsExpr,
};

pub fn generate_try_from_attribute_value_hashmap(
    struct_info: &StructInfo,
) -> proc_macro2::TokenStream {
    let struct_name = struct_info.struct_name.as_expr();
    let field_tokens = generate_attribute_value_to_fields_token(&struct_info);
    let return_token = generate_return(struct_info);
    quote! {
        impl TryFrom<std::collections::HashMap<String, dynorow::aws_sdk_dynamodb::types::AttributeValue>> for #struct_name {
            type Error = dynorow::error::Error;

            fn try_from(mut items: std::collections::HashMap<String, dynorow::aws_sdk_dynamodb::types::AttributeValue>) -> Result<Self, Self::Error> {
                #field_tokens

                #return_token
            }
        }
    }
}

fn generate_attribute_value_to_fields_token(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let mut tokens: proc_macro2::TokenStream = quote::quote! {};
    for field in struct_info.get_handled_fields() {
        let field_name_expr = field.name.as_expr();
        let field_name_str = field.name.to_string();
        let field_type_token = field.get_type_token();
        let field_key_str = field.get_key_str();
        let attribute_parse_token = generate_attribute_parse_token(&field);
        let return_token = match field.is_option {
            true => quote! {None},
            false => quote! {return Err(dynorow::error::Error::value_not_found(#field_name_str))},
        };
        quote::quote! {
            let #field_name_expr: #field_type_token = match items.remove(#field_key_str) {
                Some(#field_name_expr) => {
                    #attribute_parse_token
                },
                None => #return_token
            };
        }
        .to_tokens(&mut tokens);
    }
    tokens
}

fn generate_attribute_parse_token(field: &FieldInfo) -> proc_macro2::TokenStream {
    let field_name_expr = field.name.as_expr();
    let field_type_token = field.get_type_token();
    let field_str = field.get_type_str();

    match field.is_serde {
        true => quote! {
            dynorow::serde_json::from_str(
                #field_name_expr
                    .as_s()
                    .map_err(|e| dynorow::error::Error::parse_error(#field_name_expr.clone(), #field_str, format!("{:?}", e)))?,
            )
                .map_err(|e| dynorow::error::Error::parse_error(#field_name_expr.clone(), #field_str, format!("{:?}", e)))?
        },
        false => quote! {
            <#field_type_token as dynorow::traits::from_attribute_value::FromAttributeValue<#field_type_token>>::from_attribute_value(#field_name_expr)?
        },
    }
}

fn generate_return(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let mut fields_token = quote! {};
    for field in &struct_info.fields {
        let field_name_expr = field.name.as_expr();
        match field.ignore {
            true => quote! {#field_name_expr: Default::default(),}.to_tokens(&mut fields_token),
            false => quote! {#field_name_expr,}.to_tokens(&mut fields_token),
        }
    }
    quote! {
        Ok(Self {
            #fields_token
        })
    }
}
