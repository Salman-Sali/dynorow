use aws_sdk_dynamodb::types::AttributeValue;

pub trait IntoAttributeValue {
    fn into_attribute_value(&self) -> AttributeValue;
}

impl IntoAttributeValue for String {
    fn into_attribute_value(&self) -> AttributeValue {
        AttributeValue::S(self.clone())
    }
}

impl IntoAttributeValue for &str {
    fn into_attribute_value(&self) -> AttributeValue {
        AttributeValue::S(String::from(*self))
    }
}

impl IntoAttributeValue for i32 {
    fn into_attribute_value(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }
}

impl IntoAttributeValue for u32 {
    fn into_attribute_value(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }
}

impl IntoAttributeValue for f32 {
    fn into_attribute_value(&self) -> AttributeValue {
        AttributeValue::N(self.to_string())
    }
}

impl IntoAttributeValue for bool {
    fn into_attribute_value(&self) -> AttributeValue {
        AttributeValue::Bool(*self)
    }
}

impl IntoAttributeValue for Vec<String> {
    fn into_attribute_value(&self) -> AttributeValue {
        AttributeValue::Ss(self.clone())
    }
}
