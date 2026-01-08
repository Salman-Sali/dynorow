use super::{operations::ConditionalOpeartion, Expression};

pub struct ExpressionJoiner {
    pub expression: Expression,
    pub conditional_operation: ConditionalOpeartion,
}

impl ExpressionJoiner {
    pub fn new(expression: Expression, conditional_operation: ConditionalOpeartion) -> Self {
        Self {
            expression,
            conditional_operation,
        }
    }

    pub fn expr(self, expresssion: Expression) -> Expression {
        Expression::Binary {
            left: Box::new(self.expression),
            conditional_operation: self.conditional_operation,
            right: Box::new(expresssion),
        }
    }
}