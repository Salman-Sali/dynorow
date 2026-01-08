use std::{collections::HashMap, fmt::Debug};
use aws_sdk_dynamodb::types::WriteRequest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Value not found for `{0}`")]
    ValueNotFound(String),

    #[error(
        "Error while parsing struct field `{struct_field_name}` of type `{struct_field_type}`
        from dynamo field `{dynamo_key}` of type `{dynamo_field_type}`"
    )]
    ParseError {
        struct_field_name: String,
        struct_field_type: String,
        dynamo_key: String,
        dynamo_field_type: String,
        details: String,
    },
    #[error("Error from aws dynamodb sdk")]
    SdkError {
        info: String,
        error: String
    },
    #[error("Batch operation was abandoned after retrying.")]
    BatchOperationAbandon {unprocessed_items: HashMap<String, Vec<WriteRequest>>}
}

impl Error {
    pub fn value_not_found(field_name: &str) -> Self {
        Self::ValueNotFound(field_name.into())
    }

    pub fn parse_error(
        struct_field_name: &str,
        struct_field_type: &str,
        dynamo_key: &str,
        dynamo_field_type: String,
        details: String,
    ) -> Self {
        Self::ParseError {
            struct_field_name: struct_field_name.into(),
            struct_field_type: struct_field_type.into(),
            dynamo_key: dynamo_key.into(),
            dynamo_field_type,
            details,
        }
    }

    pub fn sdk_error(
        info: &str,
        error: impl 'static + Debug
    ) -> Self {
        Self::SdkError {
            info: info.into(),
            error: format!("{:?}", error)
        }
    }
}
