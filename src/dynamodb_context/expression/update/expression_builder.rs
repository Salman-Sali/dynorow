use std::{collections::HashSet, marker::PhantomData};

use aws_sdk_dynamodb::types::AttributeValue;

use crate::{
    dynamodb_context::expression::update::{SetOperation, UpdateExpression},
    traits::into_attribute_value::IntoAttributeValue,
};

pub struct UpdateExpressionBuilder<V> {
    pub key: String,
    pub _v: PhantomData<V>,
}

impl<V> UpdateExpressionBuilder<V> {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.into(),
            _v: Default::default(),
        }
    }

    pub fn set_new_value(self, value: impl IntoAttributeValue) -> UpdateExpression {
        UpdateExpression::new_set(SetOperation::Assign {
            key: self.key,
            value: value.into_attribute_value(),
        })
    }
}

impl<V> UpdateExpressionBuilder<Vec<V>> {
    pub fn set_list_append(self, values: impl IntoAttributeValue) -> UpdateExpression {
        UpdateExpression::new_set(SetOperation::ListAppend {
            key: self.key,
            value: values.into_attribute_value(),
        })
    }

    pub fn set_list_prepend(self, values: impl IntoAttributeValue) -> UpdateExpression {
        UpdateExpression::new_set(SetOperation::ListPrepend {
            key: self.key,
            value: values.into_attribute_value(),
        })
    }
}

impl UpdateExpressionBuilder<HashSet<String>> {
    pub fn delete_element(self, value: HashSet<String>) -> UpdateExpression {
        UpdateExpression::new_delete(self.key, value.into_attribute_value())
    }
}

impl UpdateExpressionBuilder<HashSet<i32>> {
    pub fn delete_element(self, value: HashSet<i32>) -> UpdateExpression {
        UpdateExpression::new_delete(self.key, value.into_attribute_value())
    }
}

impl UpdateExpressionBuilder<HashSet<u32>> {
    pub fn delete_element(self, value: HashSet<u32>) -> UpdateExpression {
        UpdateExpression::new_delete(self.key, value.into_attribute_value())
    }
}

impl<V> UpdateExpressionBuilder<Option<V>> {
    pub fn set_if_not_exists(self, value: impl IntoAttributeValue) -> UpdateExpression {
        UpdateExpression::new_set(SetOperation::IfNotExists {
            key: self.key,
            value: value.into_attribute_value(),
        })
    }

    pub fn remove(self) -> UpdateExpression {
        UpdateExpression::new_remove(self.key)
    }
}

impl UpdateExpressionBuilder<i32> {
    pub fn set_increment(self, increment_value: i32) -> UpdateExpression {
        UpdateExpression::new_set(SetOperation::Increment {
            key: self.key,
            value: AttributeValue::N(increment_value.to_string()),
        })
    }

    pub fn set_decrement(self, decremet_value: i32) -> UpdateExpression {
        UpdateExpression::new_set(SetOperation::Decrement {
            key: self.key,
            value: AttributeValue::N(decremet_value.to_string()),
        })
    }

    pub fn add_increment(self, increment_value: i32) -> UpdateExpression {
        UpdateExpression::new_add(self.key, AttributeValue::N(increment_value.to_string()))
    }

    pub fn add_decrement(self, decrement_value: i32) -> UpdateExpression {
        UpdateExpression::new_add(self.key, AttributeValue::N(format!("-{}", decrement_value)))
    }
}

impl UpdateExpressionBuilder<f32> {
    pub fn set_increment(self, increment_value: f32) -> UpdateExpression {
        UpdateExpression::new_set(SetOperation::Increment {
            key: self.key,
            value: AttributeValue::N(increment_value.to_string()),
        })
    }

    pub fn set_decrement(self, decremet_value: f32) -> UpdateExpression {
        UpdateExpression::new_set(SetOperation::Decrement {
            key: self.key,
            value: AttributeValue::N(decremet_value.to_string()),
        })
    }

    pub fn add_increment(self, increment_value: f32) -> UpdateExpression {
        UpdateExpression::new_add(self.key, AttributeValue::N(increment_value.to_string()))
    }

    pub fn add_decremenet(self, decrement_value: f32) -> UpdateExpression {
        UpdateExpression::new_add(self.key, AttributeValue::N(format!("-{}", decrement_value)))
    }
}

impl UpdateExpressionBuilder<u32> {
    pub fn set_increment(self, increment_value: u32) -> UpdateExpression {
        UpdateExpression::new_set(SetOperation::Increment {
            key: self.key,
            value: AttributeValue::N(increment_value.to_string()),
        })
    }

    pub fn set_decrement(self, decremet_value: u32) -> UpdateExpression {
        UpdateExpression::new_set(SetOperation::Decrement {
            key: self.key,
            value: AttributeValue::N(decremet_value.to_string()),
        })
    }

    pub fn add_increment(self, increment_value: u32) -> UpdateExpression {
        UpdateExpression::new_add(self.key, AttributeValue::N(increment_value.to_string()))
    }

    pub fn add_decremenet(self, decrement_value: u32) -> UpdateExpression {
        UpdateExpression::new_add(self.key, AttributeValue::N(format!("-{}", decrement_value)))
    }
}
