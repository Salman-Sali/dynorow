#![deny(unused_crate_dependencies)]
mod dynamodb_context;
pub mod dynamodb_sdk_extensions;
pub mod error;
pub mod key;
pub mod serde_field;
pub mod traits;

extern crate dynorow_derive;

pub use dynamodb_context::DynamodbContext;
pub use dynamodb_context::get_result_list::GetListResult;
pub use dynamodb_context::operations::Operation;

pub use dynamodb_context::expression::Expression;
pub use dynorow_derive::DynoRow;
pub use dynorow_derive::Fetchable;
pub use dynorow_derive::Insertable;
pub use dynorow_derive::Updatable;

pub use dynamodb_context::expression::expression_builder::BuildExpression;
pub use dynamodb_context::expression::expression_builder::ExpressionBuilder;

pub use aws_sdk_dynamodb;
pub use serde_json;
