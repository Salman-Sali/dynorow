use aws_sdk_dynamodb::types::DeleteRequest;

use crate::{error::Error, key::KeyValue};

use super::as_key_value::AsKeyValue;

pub trait Deletable : AsKeyValue {
    
}

pub fn into_delete_request(deletable: Box<&dyn Deletable>) -> Result<DeleteRequest, Error> {
        let key = deletable.as_key_value();
        let builder = DeleteRequest::builder();
        return match key {
            KeyValue::CompositeKey { partition_key, partition_key_value, sort_key, sort_key_value } => {
                Ok(builder
                    .key(partition_key, partition_key_value)
                    .key(sort_key, sort_key_value)
                    .build()
                    .map_err(|e| Error::sdk_error("Error while building delete request.", e)))?

            },
            KeyValue::PartitionKey { key, value } => {
                Ok(
                    builder
                        .key(key, value)
                        .build()
                        .map_err(|e| Error::sdk_error("Error while building delete request.", e))?
                )
            },
        };
}