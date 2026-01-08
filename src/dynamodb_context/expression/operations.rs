use aws_sdk_dynamodb::types::AttributeValue;

use super::ExpressionContext;

#[derive(Debug, Clone)]
pub enum ConditionalOpeartion {
    And,
    Or,
}

impl ConditionalOpeartion {
    pub fn to_string(&self) -> String {
        match self {
            ConditionalOpeartion::And => String::from("AND"),
            ConditionalOpeartion::Or => String::from("OR"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RelationalOperation {
    Equals(AttributeValue),
    Between(AttributeValue, AttributeValue)
    // In(Vec<AttributeValue>)
    // LessThan,
    // LessThanOrEqualTo,
    // GreaterThan,
    // GreaterThanOrEqualTo,
    // Between,
    // BeginsWith
}

impl RelationalOperation {
    pub fn to_string(&self, context: &mut ExpressionContext) -> String {
        match self {
            RelationalOperation::Equals(_) => format!("= :v{}", context.next()),
            RelationalOperation::Between(_, _) => format!("BETWEEN :v{} AND :v{}", context.next(), context.next()),
        }
    }
}
