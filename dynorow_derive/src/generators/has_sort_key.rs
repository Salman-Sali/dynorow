use quote::quote;

use crate::{AsExpr, StructInfo};

pub fn generate_has_sort_key(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let Some(sk_key) = struct_info.get_sk_key() else {
        return quote! {};
    };

    let struct_name_expr = struct_info.struct_name.as_expr();
    quote! {
        impl dynorow::traits::has_sort_key::HasSortKey for #struct_name_expr {
            fn get_sort_key() -> String {
                #sk_key.into()
            }
        }
    }
}
