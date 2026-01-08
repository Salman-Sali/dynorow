use aws_sdk_dynamodb::operation::{get_item::builders::GetItemFluentBuilder, update_item::builders::UpdateItemFluentBuilder};

use crate::key::KeyValue;

pub trait WithKey {
    fn with_key(self, key: &KeyValue) -> Self;
}

impl WithKey for GetItemFluentBuilder {
    fn with_key(self, key: &KeyValue) -> Self {
        match key {
            KeyValue::CompositeKey {
                partition_key,
                partition_key_value,
                sort_key,
                sort_key_value,
            } => self
                .key(partition_key, partition_key_value.clone())
                .key(sort_key, sort_key_value.clone()),
            KeyValue::PartitionKey { key, value, } => self.key(key, value.clone()),
        }
    }
}

impl WithKey for UpdateItemFluentBuilder {
    fn with_key(self, key: &KeyValue) -> Self {
        match key {
            KeyValue::CompositeKey {
                partition_key,
                partition_key_value,
                sort_key,
                sort_key_value,
            } => self
                .key(partition_key, partition_key_value.clone())
                .key(sort_key, sort_key_value.clone()),
            KeyValue::PartitionKey { key, value,} => self.key(key, value.clone()),
        }
    }
}
