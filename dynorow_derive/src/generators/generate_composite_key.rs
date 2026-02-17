use quote::{ToTokens, quote};

use crate::{AsExpr, StructInfo};

pub fn generate_generate_composite_key(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let Some(sk_key) = struct_info.get_sk_key() else {
        return quote! {};
    };

    if !struct_info.is_generated_pk_value() && !struct_info.is_static_pk_value() {
        return quote! {};
    }

    let pk_key = struct_info.get_pk_key();
    let struct_name_expr = struct_info.struct_name.as_expr();

    let mut function_parameters = quote! {};
    let mut pk_value_token = quote! {};

    if struct_info.is_generated_pk_value() {
        for part in &struct_info.pk_value_parts {
            let part_expr = part.as_expr();
            quote! {
                #part_expr: impl std::fmt::Display,
            }
            .to_tokens(&mut function_parameters);

            quote! {
                #part_expr.to_string(),
            }
            .to_tokens(&mut pk_value_token);
        }

        pk_value_token = quote! {
          #struct_name_expr::generate_pk_value(#pk_value_token).get_partition_key_value()
        };
    } else if struct_info.is_static_pk_value() {
        quote! {
            <#struct_name_expr as dynorow::traits::has_pk_value::HasStaticPkValue>::get_static_pk_value()
        }
        .to_tokens(&mut pk_value_token);
    }

    let sk_field_name = struct_info.get_sk_field().unwrap().name.as_expr();
    quote! {
        #sk_field_name: impl std::fmt::Display,
    }
    .to_tokens(&mut function_parameters);

    quote! {
        impl #struct_name_expr {
            pub fn generate_composite_key(#function_parameters) -> dynorow::key::KeyValue {

                dynorow::key::KeyValue::new_composite_key(
                    #pk_key.into(),
                    #pk_value_token,
                    #sk_key.into(),
                    #sk_field_name.to_string(),
                )
            }
        }
    }
}
