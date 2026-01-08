use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use joiner::ExpressionJoiner;
use operations::{ConditionalOpeartion, RelationalOperation};

pub mod expression_builder;
pub mod joiner;
pub mod operations;

#[derive(Debug, Clone)]
pub enum Expression {
    Bracket(Box<Expression>),
    Unit {
        key: String,
        relational_operation: RelationalOperation,
    },
    Binary {
        left: Box<Expression>,
        conditional_operation: ConditionalOpeartion,
        right: Box<Expression>,
    },
}

impl Expression {
    pub fn bracket(expression: Expression) -> Expression {
        Expression::Bracket(Box::new(expression))
    }

    pub fn unit(key: String, relational_operation: RelationalOperation) -> Expression {
        Expression::Unit {
            key,
            relational_operation,
        }
    }

    pub fn and(self) -> ExpressionJoiner {
        ExpressionJoiner::new(self, ConditionalOpeartion::And)
    }

    pub fn or(self) -> ExpressionJoiner {
        ExpressionJoiner::new(self, ConditionalOpeartion::And)
    }

    pub fn get_expression_attribute_names(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();

        match self {
            Expression::Bracket(expression) => {
                result.extend(expression.get_expression_attribute_names());
            }
            Expression::Unit {
                key,
                relational_operation: _,
            } => {
                result.insert(format!("#var_{}", key), key.clone());
            }
            Expression::Binary {
                left,
                conditional_operation: _,
                right,
            } => {
                result.extend(left.get_expression_attribute_names());
                result.extend(right.get_expression_attribute_names());
            }
        }

        return result;
    }

    pub fn get_expression_attribute_values(&self) -> HashMap<String, AttributeValue> {
        let mut context = ExpressionContext::new();
        self.get_expression_attribute_values_with_context(&mut context)
    }

    fn get_expression_attribute_values_with_context(
        &self,
        context: &mut ExpressionContext,
    ) -> HashMap<String, AttributeValue> {
        let mut result: HashMap<String, AttributeValue> = HashMap::new();

        match self {
            Expression::Bracket(expression) => {
                result.extend(expression.get_expression_attribute_values_with_context(context))
            }
            Expression::Unit {
                key: _,
                relational_operation,
            } => match relational_operation {
                RelationalOperation::Equals(attribute_value) => {
                    result.insert(format!(":v{}", context.next()), attribute_value.clone());
                }
                RelationalOperation::Between(a1, a2) => {
                    result.insert(format!(":v{}", context.next()), a1.clone());
                    result.insert(format!(":v{}", context.next()), a2.clone());
                },
            },
            Expression::Binary {
                left,
                conditional_operation: _,
                right,
            } => {
                result.extend(left.get_expression_attribute_values_with_context(context));
                result.extend(right.get_expression_attribute_values_with_context(context));
            }
        }

        return result;
    }

    pub fn to_string(&self) -> String {
        let mut context = ExpressionContext::new();
        self.to_string_with_context(&mut context)
    }

    fn to_string_with_context(&self, context: &mut ExpressionContext) -> String {
        match self {
            Expression::Bracket(expression) => {
                format!("({})", expression.to_string_with_context(context))
            }
            Expression::Unit {
                key,
                relational_operation,
            } => format!("#var_{} {}", key, relational_operation.to_string(context)),
            Expression::Binary {
                left,
                conditional_operation,
                right,
            } => format!(
                "{} {} {}",
                left.to_string_with_context(context),
                conditional_operation.to_string(),
                right.to_string_with_context(context)
            ),
        }
    }
}

pub struct ExpressionContext {
    pub val_count: u32,
}

impl ExpressionContext {
    pub fn new() -> Self {
        Self { val_count: 0 }
    }

    pub fn next(&mut self) -> u32 {
        self.val_count += 1;
        self.val_count
    }
}

// #[test]
// fn test_expression_generation() {
//     use expression_builder::BuildExpression;

//     let pk = String::from("pk");
//     let sk = String::from("sk");

//     let expression = pk
//         .string_equals("User")
//         .and()
//         .expr(sk.string_equals("user123"));

//     assert_eq!("#var_pk = :v1 AND #var_sk = :v2", expression.to_string());

//     let attribute_values = expression.get_expression_attribute_values();
//     let attribute_names = expression.get_expression_attribute_names();

//     assert_eq!(
//         r#"{":v1": S("User"), ":v2": S("user123")}"#,
//         format!("{:?}", attribute_values)
//     );
//     assert_eq!(
//         r##"{"#var_pk": "pk", "#var_sk": "sk"}"##,
//         format!("{:?}", attribute_names)
//     );
// }
