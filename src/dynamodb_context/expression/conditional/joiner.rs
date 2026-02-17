use crate::dynamodb_context::expression::conditional::ConditionalExpression;

use super::operations::ConditionalOpeartion;

pub struct ConditionalExpressionJoiner {
    pub expression: ConditionalExpression,
    pub conditional_operation: ConditionalOpeartion,
}

impl ConditionalExpressionJoiner {
    pub fn new(
        expression: ConditionalExpression,
        conditional_operation: ConditionalOpeartion,
    ) -> Self {
        Self {
            expression,
            conditional_operation,
        }
    }

    pub fn expr(self, expresssion: ConditionalExpression) -> ConditionalExpression {
        ConditionalExpression::Binary {
            left: Box::new(self.expression),
            conditional_operation: self.conditional_operation,
            right: Box::new(expresssion),
        }
    }
}
