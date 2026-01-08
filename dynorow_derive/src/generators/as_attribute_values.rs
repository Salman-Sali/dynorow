use quote::{quote, ToTokens};

use crate::{struct_info::{field_type::FieldType, StructInfo}, utils::as_expr::AsExpr};

pub fn generate_as_attribute_values(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();
    let mut fields_token = quote::quote! {};

    if struct_info.struct_has_pk() {
        let struct_pk_str = struct_info.get_pk_key();
        let struct_pk_value = struct_info.pk_value.clone().unwrap();
        quote! {
            result.insert(#struct_pk_str.into(), #struct_pk_value.into_attribute_value());
        }.to_tokens(&mut fields_token);
    }

    for field in struct_info.get_handled_fields() {
        let field_name_expr = field.name.as_expr();
        let key_str = field.get_key_str();

        let field_value_expr_token = match field.is_option {
            true => quote! {#field_name_expr},
            false => quote! {self.#field_name_expr},
        };

        let result_push_token = match field.field_type {
            FieldType::SerdeJson(_) => quote::quote! {
                result.insert(#key_str.into(), 
                    dynorow::aws_sdk_dynamodb::types::AttributeValue::S(
                        {
                            let serde_field = dynorow::serde_field::SerdeField::new(#field_value_expr_token.clone());
                            dynorow::serde_json::to_string(&serde_field).unwrap()
                        }
                    ));
            },
            _ => quote::quote! {
                result.insert(#key_str.into(), #field_value_expr_token.into_attribute_value());
            }
        };


        match field.is_option {
            true => quote! {
                if let Some(#field_name_expr) = &self.#field_name_expr {
                   #result_push_token 
                }
            },
            false => result_push_token,
        }.to_tokens(&mut fields_token);
    }
    quote::quote! {
        impl dynorow::traits::as_attribute_key_values::AsAttributeKeyValues for #struct_name_expr {
            fn as_attribute_key_values(&self) -> std::collections::HashMap<String, dynorow::aws_sdk_dynamodb::types::AttributeValue> {
                use dynorow::traits::into_attribute_value::*;
                let mut result = std::collections::HashMap::<String, dynorow::aws_sdk_dynamodb::types::AttributeValue>::new();

                #fields_token

                return result;
            }
        }
    }.into()
}