#![deny(unused_crate_dependencies)]
mod generators;
mod struct_info;
mod utils;

use generators::{
    as_attribute_values::generate_as_attribute_values, as_key_value::generate_as_key_value_token,
    as_projection::generate_as_projection,
    conditional_expression_builder::generate_conditional_expression_builder_token,
    has_key::generate_has_key_token, has_static_pk_value::generate_has_pk_value_token,
    has_table_name::generate_has_table_name,
    try_from_attribute_value_hashmap::generate_try_from_attribute_value_hashmap,
    try_from_get_item_output::generate_try_from_get_item_output,
};
use proc_macro::TokenStream;
use quote::quote;
use struct_info::StructInfo;
use syn::{DeriveInput, parse_macro_input};
use utils::as_expr::AsExpr;

use crate::{
    generate_composite_key::generate_generate_composite_key,
    generators::{
        generate_composite_key, generate_pk_value::generate_generate_pk_value,
        has_pk_value_template::generate_has_pk_value_template, has_sort_key::generate_has_sort_key,
        update_expression_builder::generate_update_expression_builder_token,
        update_expression_builder_for_dynomap::generate_dynomap_update_expression_builder_token,
    },
};

#[proc_macro_derive(DynoRow, attributes(dynorow))]
pub fn dynorow_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_info = StructInfo::new(input, true);

    let try_from_attribute_value_hashmap = generate_try_from_attribute_value_hashmap(&struct_info);
    let try_from_get_item_output = generate_try_from_get_item_output(&struct_info);
    let as_attribute_values = generate_as_attribute_values(&struct_info);
    let as_key_token = generate_as_key_value_token(&struct_info);
    let as_projection_token = generate_as_projection(&struct_info);
    let has_table_name_token = generate_has_table_name(&struct_info);
    let has_key = generate_has_key_token(&struct_info);
    let has_pk_value = generate_has_pk_value_token(&struct_info);
    let conditional_expression_builder =
        generate_conditional_expression_builder_token(&struct_info);
    let update_expression_builder = generate_update_expression_builder_token(&struct_info);
    let dyno_map_update_expression = generate_dynomap_update_expression_builder_token(&struct_info);

    let struct_name_expr = struct_info.struct_name.as_expr();
    let generate_pk_value = generate_generate_pk_value(&struct_info);
    let pk_value_template = generate_has_pk_value_template(&struct_info);
    let has_sort_key = generate_has_sort_key(&struct_info);
    let generate_composite_key = generate_generate_composite_key(&struct_info);
    quote! {
        #has_sort_key

        #generate_pk_value

        #generate_composite_key

        #pk_value_template

        #conditional_expression_builder

        #dyno_map_update_expression

        #update_expression_builder

        #has_pk_value

        #try_from_attribute_value_hashmap

        #has_key

        #try_from_get_item_output

        #as_attribute_values

        #as_key_token

        #as_projection_token

        #has_table_name_token

        impl dynorow::traits::dyno_map_trait::DynoMapTrait for #struct_name_expr {}
    }
    .into()
}

#[proc_macro_derive(DynoMap, attributes(dynorow))]
pub fn dynomap_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_info = StructInfo::new(input, false);
    let struct_name_expr = struct_info.struct_name.as_expr();

    let try_from_attribute_value_hashmap = generate_try_from_attribute_value_hashmap(&struct_info);
    let as_attribute_values = generate_as_attribute_values(&struct_info);
    let dyno_map_update_expression = generate_dynomap_update_expression_builder_token(&struct_info);

    quote! {
        #dyno_map_update_expression

        #try_from_attribute_value_hashmap

        #as_attribute_values

        impl dynorow::traits::dyno_map_trait::DynoMapTrait for #struct_name_expr {}
    }
    .into()
}

#[proc_macro_derive(Insertable)]
pub fn insertable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_info = StructInfo::new(input, true);
    let struct_name_expr = struct_info.struct_name.as_expr();
    quote! {
        impl dynorow::traits::insertable::Insertable for #struct_name_expr {

        }

    }
    .into()
}

#[proc_macro_derive(Updatable)]
pub fn updatable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_info = StructInfo::new(input, true);
    let struct_name_expr = struct_info.struct_name.as_expr();
    quote! {
        impl dynorow::traits::updatable::Updatable for #struct_name_expr {

        }

    }
    .into()
}

#[proc_macro_derive(Fetchable)]
pub fn fetchable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_info = StructInfo::new(input, true);
    let struct_name_expr = struct_info.struct_name.as_expr();
    quote! {
        impl dynorow::traits::fetchable::Fetchable for #struct_name_expr {
            type Error = dynorow::error::Error;
        }

    }
    .into()
}
