use crate::{
    struct_info::StructInfo,
    utils::as_expr::AsExpr,
};

pub fn generate_try_from_get_item_output(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name = struct_info.struct_name.as_expr();
    
    quote::quote! {
        impl TryFrom<dynorow::aws_sdk_dynamodb::operation::get_item::GetItemOutput> for #struct_name {
            type Error = dynorow::error::Error;

            fn try_from(value: dynorow::aws_sdk_dynamodb::operation::get_item::GetItemOutput) -> Result<Self, Self::Error> {
                let Some(items) = value.item else {
                    return Err(dynorow::error::Error::value_not_found("GetItemOuput.item".into()));
                };
                
                Self::try_from(items)
            }
        }
    }.into()
}

