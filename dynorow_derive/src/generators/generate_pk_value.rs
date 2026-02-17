use quote::{ToTokens, quote};

use crate::{AsExpr, StructInfo};

pub fn generate_generate_pk_value(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name = struct_info.struct_name.as_expr();

    if !struct_info.is_generated_pk_value() {
        if struct_info.is_static_pk_value() {
            let pk_key = struct_info.pk.clone().unwrap();

            return quote! {
                impl #struct_name {
                    pub fn generate_pk_value() -> dynorow::key::KeyValue {
                        dynorow::key::KeyValue::new_partition_key(
                            #pk_key.into(),
                            <#struct_name as dynorow::traits::has_pk_value::HasStaticPkValue>::get_static_pk_value()
                        )

                    }
                }
            };
        }
        return quote! {};
    }

    let mut generate_function_parameters = quote! {};
    let mut generate_format_parameters = quote! {};
    let mut as_format_parameters = quote! {};
    for part in &struct_info.pk_value_parts {
        let part_expr = part.as_expr();
        quote! {
            #part_expr: impl std::fmt::Display,
        }
        .to_tokens(&mut generate_function_parameters);

        quote! {
            self.#part_expr,
        }
        .to_tokens(&mut as_format_parameters);

        quote! {
            #part_expr,
        }
        .to_tokens(&mut generate_format_parameters);
    }
    let pk_key = struct_info.pk.clone().unwrap();

    let pk_value = struct_info.pk_value.clone().unwrap();
    quote! {
        impl #struct_name {
            pub fn generate_pk_value(#generate_function_parameters) -> dynorow::key::KeyValue {
                dynorow::key::KeyValue::new_partition_key(
                    #pk_key.into(),
                    format!(#pk_value, #generate_format_parameters))

            }

            pub fn as_pk_value(&self) -> dynorow::key::KeyValue {
                dynorow::key::KeyValue::new_partition_key(
                    #pk_key.into(),
                    format!(#pk_value, #as_format_parameters))

            }
        }
    }
}
