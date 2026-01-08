use aws_sdk_dynamodb::types::AttributeValue;

pub trait AsAttributeKeyValues {
    fn as_attribute_key_values(&self) -> std::collections::HashMap<String, AttributeValue>;
}