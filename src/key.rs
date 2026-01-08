use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;

use crate::{
    Expression, dynamodb_context::expression::expression_builder::BuildExpression, error::Error,
};

#[derive(Debug, Clone)]
pub enum Key {
    CompositeKey {
        partition_key: String,
        sort_key: String,
    },
    PartitionKey {
        key: String,
    },
}

impl Key {
    pub fn new_composite_key(pk: &str, sk: &str) -> Key {
        Key::CompositeKey {
            partition_key: pk.into(),
            sort_key: sk.into(),
        }
    }
    pub fn get_partition_key(&self) -> String {
        match self {
            Key::CompositeKey {
                partition_key,
                sort_key: _,
            } => partition_key.clone(),
            Key::PartitionKey { key } => key.clone(),
        }
    }

    pub fn get_sort_key(&self) -> Option<String> {
        match self {
            Key::CompositeKey {
                partition_key: _,
                sort_key,
            } => Some(sort_key.clone()),
            Key::PartitionKey { key: _ } => None,
        }
    }

    pub fn into_partition_key_value(self, partition_key_value: String) -> KeyValue {
        let partition_key = match self {
            Key::CompositeKey {
                partition_key,
                sort_key: _,
            } => partition_key,
            Key::PartitionKey { key } => key,
        };

        KeyValue::new_partition_key(partition_key, partition_key_value)
    }

    pub fn is_equal_to(&self, key_hash_map: &HashMap<String, AttributeValue>) -> bool {
        match self {
            Key::CompositeKey {
                partition_key,
                sort_key,
            } => {
                key_hash_map.len() == 2
                    && key_hash_map.contains_key(partition_key)
                    && key_hash_map.contains_key(sort_key)
            }
            Key::PartitionKey { key } => key_hash_map.len() == 1 && key_hash_map.contains_key(key),
        }
    }
}

#[derive(Debug, Clone)]
pub enum KeyValue {
    CompositeKey {
        partition_key: String,
        partition_key_value: AttributeValue,
        sort_key: String,
        sort_key_value: AttributeValue,
    },
    PartitionKey {
        key: String,
        value: AttributeValue,
    },
}

impl KeyValue {
    pub fn new_composite_key(
        partition_key: String,
        partition_key_value: String,
        sort_key: String,
        sort_key_value: String,
    ) -> Self {
        Self::CompositeKey {
            partition_key,
            partition_key_value: AttributeValue::S(partition_key_value),
            sort_key: sort_key.into(),
            sort_key_value: AttributeValue::S(sort_key_value),
        }
    }

    pub fn new_partition_key(key: String, value: String) -> Self {
        Self::PartitionKey {
            key,
            value: AttributeValue::S(value),
        }
    }

    pub fn project_key(&self) -> String {
        match self {
            KeyValue::CompositeKey {
                partition_key,
                partition_key_value: _,
                sort_key,
                sort_key_value: _,
            } => format!("{}, {}", partition_key, sort_key),
            KeyValue::PartitionKey { key, value: _ } => key.clone(),
        }
    }

    pub fn from_hash_map(
        hash_map: HashMap<String, AttributeValue>,
        key: Key,
    ) -> Result<KeyValue, Error> {
        let partition_key = key.get_partition_key();
        let sort_key = key.get_sort_key();
        let Some(partition_key_value) = hash_map.get(&partition_key) else {
            return Err(Error::ValueNotFound(partition_key));
        };

        if let Some(sort_key) = sort_key {
            let Some(sort_key_value) = hash_map.get(&sort_key) else {
                return Err(Error::ValueNotFound(sort_key));
            };

            return Ok(KeyValue::CompositeKey {
                partition_key: partition_key,
                partition_key_value: partition_key_value.clone(),
                sort_key: sort_key,
                sort_key_value: sort_key_value.clone(),
            });
        }

        return Ok(KeyValue::PartitionKey {
            key: partition_key,
            value: partition_key_value.clone(),
        });
    }

    pub fn into_hash_map(self) -> HashMap<String, AttributeValue> {
        let mut result: HashMap<String, AttributeValue> = HashMap::new();
        match self {
            KeyValue::CompositeKey {
                partition_key,
                partition_key_value,
                sort_key,
                sort_key_value,
            } => {
                result.insert(partition_key, partition_key_value);
                result.insert(sort_key, sort_key_value);
            }
            KeyValue::PartitionKey { key, value } => {
                result.insert(key, value);
            }
        }
        return result;
    }

    pub fn to_key(&self) -> Key {
        match self {
            KeyValue::CompositeKey {
                partition_key,
                partition_key_value: _,
                sort_key,
                sort_key_value: _,
            } => Key::CompositeKey {
                partition_key: partition_key.clone(),
                sort_key: sort_key.clone(),
            },
            KeyValue::PartitionKey { key, value: _ } => Key::PartitionKey { key: key.clone() },
        }
    }

    pub fn get_sort_key_value(&self) -> Option<AttributeValue> {
        match self {
            KeyValue::CompositeKey {
                partition_key: _,
                partition_key_value: _,
                sort_key: _,
                sort_key_value,
            } => Some(sort_key_value.clone()),
            _ => None,
        }
    }

    pub fn into_conditional_expression(self) -> Expression {
        match self {
            KeyValue::CompositeKey {
                partition_key,
                partition_key_value,
                sort_key,
                sort_key_value,
            } => partition_key
                .equals(partition_key_value.clone())
                .and()
                .expr(sort_key.equals(sort_key_value.clone())),
            KeyValue::PartitionKey { key, value } => key.equals(value.clone()),
        }
    }

    pub fn is_partition_key_value_partial_equal(&self, partial_pk_value: &str) -> bool {
        match self {
            KeyValue::CompositeKey {
                partition_key: _,
                partition_key_value,
                sort_key: _,
                sort_key_value: _,
            } => {
                if let Ok(value) = partition_key_value.as_s() {
                    return value.contains(partial_pk_value);
                }
                return false;
            },
            KeyValue::PartitionKey { key: _, value } => {
                if let Ok(value) = value.as_s() {
                    return value.contains(partial_pk_value);
                }
                return false;
                
            },
        }
    }

    pub fn is_partition_key_value_equal(&self, pk_value: &str) -> bool {
       match self {
            KeyValue::CompositeKey {
                partition_key: _,
                partition_key_value,
                sort_key: _,
                sort_key_value: _,
            } => {
                if let Ok(value) = partition_key_value.as_s() {
                    return value == pk_value;
                }
                return false;
            },
            KeyValue::PartitionKey { key: _, value } => {
                if let Ok(value) = value.as_s() {
                    return value == pk_value;
                }
                return false;
                
            },
        }
    }
}
