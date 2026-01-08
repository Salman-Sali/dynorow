use quote::{quote, ToTokens};
use syn::Expr;

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_as_key_value_token(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();
    let pk_key = struct_info.get_pk_key();
    let sk_key = struct_info.get_sk_key();
    let pk_value = struct_info.pk_value.clone();
    let pk_value_available = pk_value.is_some();
    let sk_key_available = sk_key.is_some();    

    let mut static_key_generator_caller_token = quote! {};
    let static_key_generator_token = if !pk_value_available && !sk_key_available {
        let pk_field_name_expr = struct_info.get_pk_field().unwrap().name.as_expr();
        quote! {
            use dynorow::traits::as_key_value::AsPartitionKeyValue;
            #struct_name_expr::as_partition_key_value(self.#pk_field_name_expr.clone())
        }.to_tokens(&mut static_key_generator_caller_token);

        generate_as_partition_key_value(&struct_name_expr, &pk_key)
    } else if !pk_value_available && sk_key_available {
        let pk_field_name_expr = struct_info.get_pk_field().unwrap().name.as_expr();
        let sk_field_name_expr = struct_info.get_sk_field().unwrap().name.as_expr();
        quote! {
            use dynorow::traits::as_key_value::AsCompositeKeyValue;
            #struct_name_expr::as_composite_key_value(self.#pk_field_name_expr.clone(), self.#sk_field_name_expr.clone())
        }.to_tokens(&mut static_key_generator_caller_token);

        generate_as_composite_key_value(&struct_name_expr, &pk_key, &sk_key.unwrap())
    } else if pk_value_available && !sk_key_available {
        quote! {
            use dynorow::traits::as_key_value::AsValueAvailablePkValue;
            #struct_name_expr::as_value_available_pk()
        }.to_tokens(&mut static_key_generator_caller_token);

        generate_as_value_available_pk(&struct_name_expr, &pk_key, &pk_value.unwrap())
    } else {
        let sk_field_name_expr = struct_info.get_sk_field().unwrap().name.as_expr();
        quote! {
            use dynorow::traits::as_key_value::AsPkAvailableCompositeKeyValue;
            #struct_name_expr::as_pk_available_composite_key_value(self.#sk_field_name_expr.clone())
        }.to_tokens(&mut static_key_generator_caller_token);

        generate_as_pk_available_composite_key_value(&struct_name_expr, &pk_key, &sk_key.unwrap(), &pk_value.unwrap())
    };

    quote! {
        #static_key_generator_token
        impl dynorow::traits::as_key_value::AsKeyValue for #struct_name_expr {
            fn as_key_value(&self) -> dynorow::key::KeyValue {
                #static_key_generator_caller_token
            }
        }
    }
}

fn generate_as_pk_available_composite_key_value(struct_name_expr: &Expr, pk_key: &String, sk_key: &String, pk_value: &String) -> proc_macro2::TokenStream {
    quote::quote! {
        impl dynorow::traits::as_key_value::AsPkAvailableCompositeKeyValue for #struct_name_expr {
            fn as_pk_available_composite_key_value(sort_key_value: String) -> dynorow::key::KeyValue {
                dynorow::key::KeyValue::new_composite_key(#pk_key.into(), #pk_value.into(), #sk_key.into(), sort_key_value)
            }
        }
    }
}

fn generate_as_composite_key_value(struct_name_expr: &Expr, pk_key: &String, sk_key: &String) -> proc_macro2::TokenStream {
    quote::quote! {
        impl dynorow::traits::as_key_value::AsCompositeKeyValue for #struct_name_expr {
            fn as_composite_key_value(partition_key_value: String, sort_key_value: String) -> dynorow::key::KeyValue {
                dynorow::key::KeyValue::new_composite_key(#pk_key.into(), partition_key_value, #sk_key.into(), sort_key_value)
            }
        }
    }
}

fn generate_as_partition_key_value(struct_name_expr: &Expr, pk_key: &String) -> proc_macro2::TokenStream {
    quote::quote! {
        impl dynorow::traits::as_key_value::AsPartitionKeyValue for #struct_name_expr {
            fn as_partition_key_value(partition_key_value: String) -> dynorow::key::KeyValue {
                dynorow::key::KeyValue::new_partition_key_value(#pk_key.into(), partition_key_value)
            }
        }
    }
}

fn generate_as_value_available_pk(struct_name_expr: &Expr, pk_key: &String, pk_value: &String) -> proc_macro2::TokenStream {
    quote! {
        impl dynorow::traits::as_key_value::AsValueAvailablePkValue<#struct_name_expr> for #struct_name_expr {
            fn as_value_available_pk() -> dynorow::key::KeyValue {
                dynorow::key::KeyValue::new_partition_key_value(#pk_key.into(), #pk_value.into())
            }
        }        
    }
}