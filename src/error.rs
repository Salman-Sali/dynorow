use aws_sdk_dynamodb::types::{AttributeValue, WriteRequest};
use std::{collections::HashMap, fmt::Debug};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Value not found for `{0}`")]
    ValueNotFound(String),
    #[error("Error while converting value into dynamodb attribute value object.")]
    IntoAttributeError(String),
    #[error("Error while parsing '{value:?}' to field type '{field_type}'.")]
    ParseError {
        value: AttributeValue,
        field_type: String,
        error_debug: String,
    },
    #[error("Error from aws dynamodb sdk")]
    SdkError { info: String, error: String },
    #[error("Batch operation was abandoned after retrying.")]
    BatchOperationAbandon {
        unprocessed_items: HashMap<String, Vec<WriteRequest>>,
    },
}

impl Error {
    pub fn value_not_found(field_name: &str) -> Self {
        Self::ValueNotFound(field_name.into())
    }

    pub fn parse_error(value: AttributeValue, field_type: &str, error_debug: String) -> Self {
        Self::ParseError {
            value,
            field_type: field_type.to_string(),
            error_debug,
        }
    }

    pub fn sdk_error(info: &str, error: impl 'static + Debug) -> Self {
        Self::SdkError {
            info: info.into(),
            error: format!("{:?}", error),
        }
    }
}
