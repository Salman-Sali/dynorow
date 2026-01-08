use aws_sdk_dynamodb::types::AttributeValue;

use super::{Expression, operations::RelationalOperation};

pub struct ExpressionBuilder {
    pub key: String,
}

impl ExpressionBuilder {
    pub fn new(key: &str) -> Self {
        Self { key: key.into() }
    }
}

impl BuildExpression for ExpressionBuilder {
    fn string_equals(self, value: &str) -> Expression {
        self.key.string_equals(value)
    }
    
    fn equals(self, value: AttributeValue) -> Expression {
        self.key.equals(value)
    }

    fn string_between(self, a1: String, a2: String) -> Expression {
        self.key.string_between(a1, a2)
    }

    fn between(self, a1: AttributeValue, a2: AttributeValue) -> Expression {
        self.key.between(a1, a2)
    }
}

impl BuildExpression for String {
    fn string_equals(self, value: &str) -> Expression {
        Expression::unit(
            self,
            RelationalOperation::Equals(AttributeValue::S(value.into())),
        )
    }
    
    fn equals(self, value: AttributeValue) -> Expression {
        Expression::unit(
            self,
            RelationalOperation::Equals(value),
        )
    }
    
    fn string_between(self, a1: String, a2: String) -> Expression {
        let a1 = AttributeValue::S(a1);
        let a2 = AttributeValue::S(a2);
        self.between(a1, a2)
    }

    fn between(self, a1: AttributeValue, a2: AttributeValue) -> Expression {
        Expression::unit(self, RelationalOperation::Between(a1, a2))
    }
}

pub trait BuildExpression {
    #[allow(unused)]
    fn string_equals(self, value: &str) -> Expression;
    fn equals(self, value: AttributeValue) -> Expression;
    fn between(self, a1: AttributeValue, a2: AttributeValue) -> Expression;
    fn string_between(self, a1: String, a2: String) -> Expression;
}
