use aws_sdk_dynamodb::types::AttributeValue;

use crate::{
    dynamodb_context::expression::conditional::{
        ConditionalExpression, operations::RelationalOperation,
    },
    traits::into_attribute_value::IntoAttributeValue,
};

pub struct ConditionalExpressionBuilder {
    pub key: String,
}

impl ConditionalExpressionBuilder {
    pub fn new(key: &str) -> Self {
        Self { key: key.into() }
    }
}

impl BuildConditionalExpression for ConditionalExpressionBuilder {
    fn string_equals(self, value: &str) -> ConditionalExpression {
        self.key.string_equals(value)
    }

    fn equals(self, value: impl IntoAttributeValue) -> ConditionalExpression {
        self.key.equals(value)
    }

    fn string_between(self, a1: String, a2: String) -> ConditionalExpression {
        self.key.string_between(a1, a2)
    }

    fn between(
        self,
        a1: impl IntoAttributeValue,
        a2: impl IntoAttributeValue,
    ) -> ConditionalExpression {
        self.key.between(a1, a2)
    }
}

impl BuildConditionalExpression for String {
    fn string_equals(self, value: &str) -> ConditionalExpression {
        ConditionalExpression::unit(
            self,
            RelationalOperation::Equals(AttributeValue::S(value.into())),
        )
    }

    fn equals(self, value: impl IntoAttributeValue) -> ConditionalExpression {
        ConditionalExpression::unit(
            self,
            RelationalOperation::Equals(value.into_attribute_value()),
        )
    }

    fn string_between(self, a1: String, a2: String) -> ConditionalExpression {
        self.between(a1, a2)
    }

    fn between(
        self,
        a1: impl IntoAttributeValue,
        a2: impl IntoAttributeValue,
    ) -> ConditionalExpression {
        ConditionalExpression::unit(
            self,
            RelationalOperation::Between(a1.into_attribute_value(), a2.into_attribute_value()),
        )
    }
}

pub trait BuildConditionalExpression {
    #[allow(unused)]
    fn string_equals(self, value: &str) -> ConditionalExpression;
    fn equals(self, value: impl IntoAttributeValue) -> ConditionalExpression;
    fn between(
        self,
        a1: impl IntoAttributeValue,
        a2: impl IntoAttributeValue,
    ) -> ConditionalExpression;
    fn string_between(self, a1: String, a2: String) -> ConditionalExpression;
}
