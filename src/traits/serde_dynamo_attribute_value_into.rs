use std::collections::HashMap;

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};

pub trait SerdeDynamoAttributeValueInto {
    fn into_aws_attribute_value(self) -> aws_sdk_dynamodb::types::AttributeValue;
}

impl SerdeDynamoAttributeValueInto for serde_dynamo::AttributeValue {
    fn into_aws_attribute_value(self) -> aws_sdk_dynamodb::types::AttributeValue {
        match self {
            serde_dynamo::AttributeValue::N(x) => AttributeValue::N(x),
            serde_dynamo::AttributeValue::S(x) => AttributeValue::S(x),
            serde_dynamo::AttributeValue::Bool(x) => AttributeValue::Bool(x),
            serde_dynamo::AttributeValue::B(items) => AttributeValue::B(Blob::from(items)),
            serde_dynamo::AttributeValue::Null(x) => AttributeValue::Null(x),
            serde_dynamo::AttributeValue::M(hash_map) => {
                AttributeValue::M(hash_map.into_aws_attribute_value_hashmap())
            }
            serde_dynamo::AttributeValue::L(attribute_values) => AttributeValue::L(
                attribute_values
                    .into_iter()
                    .map(|x| x.into_aws_attribute_value())
                    .collect(),
            ),
            serde_dynamo::AttributeValue::Ss(items) => AttributeValue::Ss(items),
            serde_dynamo::AttributeValue::Ns(items) => AttributeValue::Ns(items),
            serde_dynamo::AttributeValue::Bs(items) => {
                AttributeValue::Bs(items.into_iter().map(|x| Blob::from(x)).collect())
            }
        }
    }
}

pub trait SerdeDynamoAttributeValueHashMapInto {
    fn into_aws_attribute_value_hashmap(
        self,
    ) -> HashMap<String, aws_sdk_dynamodb::types::AttributeValue>;
}

impl SerdeDynamoAttributeValueHashMapInto for HashMap<String, serde_dynamo::AttributeValue> {
    fn into_aws_attribute_value_hashmap(
        self,
    ) -> HashMap<String, aws_sdk_dynamodb::types::AttributeValue> {
        let mut result: HashMap<String, aws_sdk_dynamodb::types::AttributeValue> = HashMap::new();
        for item in self {
            result.insert(item.0, item.1.into_aws_attribute_value());
        }
        return result;
    }
}
