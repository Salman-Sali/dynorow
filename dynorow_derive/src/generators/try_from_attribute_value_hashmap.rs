use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{
    struct_info::{StructInfo, field_info::FieldInfo, field_type::FieldType},
    utils::as_expr::AsExpr,
};

pub fn generate_try_from_attribute_value_hashmap(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name = struct_info.struct_name.as_expr();
    let field_tokens = generate_attribute_value_to_fields_token(&struct_info);
    let return_token = generate_return(struct_info);
    quote! {
        impl TryFrom<std::collections::HashMap<String, dynorow::aws_sdk_dynamodb::types::AttributeValue>> for #struct_name {
            type Error = dynorow::error::Error;

            fn try_from(items: std::collections::HashMap<String, dynorow::aws_sdk_dynamodb::types::AttributeValue>) -> Result<Self, Self::Error> {
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
        let is_null_check_token = match field.is_option {
            true => quote! {Some(verified_on) if verified_on.is_null() => None,},
            false => quote! {},
        };
        quote::quote! {
            let #field_name_expr: #field_type_token = match items.get(#field_key_str) {
                #is_null_check_token
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
    let field_name_str = field.name.to_string();
    let field_type_str = field.get_type_str();
    let field_key_str = field.get_key_str();
    //let field_type_token = field.get_type_token();
    let field_type_without_option = field.field_syn_type.clone();

    let matching_token: TokenStream;
    let parsing_return_token = match field.is_option {
        true => quote!{Some(parsed_value)},
        false => quote! {parsed_value},
    };
    let mut parsing_token: TokenStream = quote::quote! {
                match x.parse() {
                    Ok(parsed_value) => #parsing_return_token,
                    Err(e) => return Err(dynorow::error::Error::parse_error(#field_name_str, #field_type_str, #field_key_str, format!("{:?}", #field_name_expr), e.to_string()))
                }
            };
    match field.field_type {
        FieldType::SerdeJson(_) => {
            matching_token = quote! {as_s()};
            parsing_token = quote::quote! {
                match &x.parse::<dynorow::serde_field::SerdeField::<#field_type_without_option>>() {
                    Ok(parsed_value) => {
                        let parsed_value = parsed_value.value.clone();
                        #parsing_return_token
                    },
                    Err(e) => return Err(dynorow::error::Error::parse_error(#field_name_str, #field_type_str, #field_key_str, format!("{:?}", #field_name_expr), e.to_string()))
                }
            };
        },
        FieldType::VecString => {
            matching_token = quote! {as_ss()};
            parsing_token = quote! {x.clone()}
        }
        FieldType::String => {
            matching_token = quote::quote! {as_s()};
        },
        FieldType::f32 | FieldType::i32 | FieldType::u32 => {
            matching_token = quote::quote! {as_n()};
        },
        FieldType::bool => {
            matching_token = quote::quote! {as_bool()};
            parsing_token = quote! {*x}
        }
    }

    quote::quote! {
        match #field_name_expr.#matching_token {
            Ok(x) => #parsing_token,
            Err(_) => return Err(dynorow::error::Error::parse_error(#field_name_str, #field_type_str, #field_key_str, format!("{:?}", #field_name_expr), String::new()))
        }
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
