use std::collections::HashSet;

use aws_sdk_dynamodb::types::AttributeValue;

use crate::traits::as_attribute_key_values::AsAttributeKeyValues;

pub trait IntoAttributeValue {
    fn into_attribute_value(&self) -> AttributeValue;
}

impl IntoAttributeValue for AttributeValue {
    fn into_attribute_value(&self) -> AttributeValue {
        self.clone()
    }
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

impl IntoAttributeValue for HashSet<String> {
    fn into_attribute_value(&self) -> AttributeValue {
        if self.is_empty() {
            AttributeValue::Null(true)
        } else {
            AttributeValue::Ss(self.iter().map(|x| x.clone()).collect())
        }
    }
}

impl IntoAttributeValue for HashSet<i32> {
    fn into_attribute_value(&self) -> AttributeValue {
        if self.is_empty() {
            AttributeValue::Null(true)
        } else {
            AttributeValue::Ns(self.iter().map(|x| x.to_string()).collect())
        }
    }
}

impl IntoAttributeValue for HashSet<u32> {
    fn into_attribute_value(&self) -> AttributeValue {
        if self.is_empty() {
            AttributeValue::Null(true)
        } else {
            AttributeValue::Ns(self.iter().map(|x| x.to_string()).collect())
        }
    }
}

impl<T> IntoAttributeValue for Vec<T>
where
    T: IntoAttributeValue,
{
    fn into_attribute_value(&self) -> AttributeValue {
        AttributeValue::L(self.iter().map(|x| x.into_attribute_value()).collect())
    }
}

impl<T> IntoAttributeValue for T
where
    T: AsAttributeKeyValues,
{
    fn into_attribute_value(&self) -> AttributeValue {
        AttributeValue::M(self.as_attribute_key_values())
    }
}

impl<T> IntoAttributeValue for Option<T>
where
    T: IntoAttributeValue,
{
    fn into_attribute_value(&self) -> AttributeValue {
        match self {
            Some(x) => x.into_attribute_value(),
            None => AttributeValue::Null(true),
        }
    }
}
