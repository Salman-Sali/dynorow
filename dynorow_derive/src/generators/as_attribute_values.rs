use quote::{ToTokens, quote};

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_as_attribute_values(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();
    let mut fields_token = quote::quote! {};

    if struct_info.struct_has_pk() {
        let struct_pk_str = struct_info.get_pk_key();
        let struct_pk_value = struct_info.pk_value.clone().unwrap();
        if struct_info.is_generated_pk_value() {
            quote! {
                result.insert(#struct_pk_str.into(), self.as_pk_value().get_partition_key_value());
            }
            .to_tokens(&mut fields_token);
        } else {
            quote! {
                result.insert(#struct_pk_str.into(), #struct_pk_value.into_attribute_value());
            }
            .to_tokens(&mut fields_token);
        }
    }

    for field in struct_info.get_handled_fields() {
        let field_name_expr = field.name.as_expr();
        let key_str = field.get_key_str();
        let field_type_expr = field.get_type_token();

        match field.is_serde {
            true => quote! {
                result.insert(#key_str.into(),
                    aws_sdk_dynamodb::types::AttributeValue::S(
                        dynorow::serde_json::to_string(&self.#field_name_expr)
                            .expect("Should be able to generate json from value.")
                    ));
            },
            false => {
                quote! {
                    result.insert(#key_str.into(),
                       <#field_type_expr as dynorow::traits::into_attribute_value::IntoAttributeValue>::into_attribute_value(&self.#field_name_expr));
                }
            }
        }.to_tokens(&mut fields_token);
    }
    quote! {
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
