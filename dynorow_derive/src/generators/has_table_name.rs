use quote::quote;

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_has_table_name(struct_info: &StructInfo) -> proc_macro2::TokenStream {

    let Some(table_name_provider) = &struct_info.table_name_provider else {
        return quote! {};
    };

    let struct_name_expr = struct_info.struct_name.as_expr();
    let table_name_provider_expr = table_name_provider.as_expr();

    quote! {
        impl dynorow::traits::has_table_name::HasTableName for #struct_name_expr {
            fn get_table_name() -> String {
                #table_name_provider_expr.clone()
            }
        }
    }
}