pub mod expression_builder;
pub mod joiner;
pub mod operations;

use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;

use crate::dynamodb_context::expression::{
    AsVariable, ExpressionContext,
    conditional::{
        joiner::ConditionalExpressionJoiner,
        operations::{ConditionalOpeartion, RelationalOperation},
    },
};

// pending refactor
#[derive(Debug, Clone)]
pub enum ConditionalExpression {
    Bracket(Box<ConditionalExpression>),
    Unit {
        key: String,
        relational_operation: RelationalOperation,
    },
    Binary {
        left: Box<ConditionalExpression>,
        conditional_operation: ConditionalOpeartion,
        right: Box<ConditionalExpression>,
    },
}

impl ConditionalExpression {
    pub fn bracket(expression: ConditionalExpression) -> ConditionalExpression {
        ConditionalExpression::Bracket(Box::new(expression))
    }

    pub fn unit(key: String, relational_operation: RelationalOperation) -> ConditionalExpression {
        ConditionalExpression::Unit {
            key,
            relational_operation,
        }
    }

    pub fn and(self) -> ConditionalExpressionJoiner {
        ConditionalExpressionJoiner::new(self, ConditionalOpeartion::And)
    }

    pub fn or(self) -> ConditionalExpressionJoiner {
        ConditionalExpressionJoiner::new(self, ConditionalOpeartion::And)
    }

    pub fn get_expression_attribute_names(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();

        match self {
            ConditionalExpression::Bracket(expression) => {
                result.extend(expression.get_expression_attribute_names());
            }
            ConditionalExpression::Unit {
                key,
                relational_operation: _,
            } => {
                result.insert(key.as_variable(), key.clone());
            }
            ConditionalExpression::Binary {
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
        let mut context = ExpressionContext::new("vc");
        self.get_expression_attribute_values_with_context(&mut context)
    }

    fn get_expression_attribute_values_with_context(
        &self,
        context: &mut ExpressionContext,
    ) -> HashMap<String, AttributeValue> {
        let mut result: HashMap<String, AttributeValue> = HashMap::new();

        match self {
            ConditionalExpression::Bracket(expression) => {
                result.extend(expression.get_expression_attribute_values_with_context(context))
            }
            ConditionalExpression::Unit {
                key: _,
                relational_operation,
            } => match relational_operation {
                RelationalOperation::Equals(attribute_value) => {
                    result.insert(context.next(), attribute_value.clone());
                }
                RelationalOperation::Between(a1, a2) => {
                    result.insert(context.next(), a1.clone());
                    result.insert(context.next(), a2.clone());
                }
            },
            ConditionalExpression::Binary {
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
        let mut context = ExpressionContext::new("vc");
        self.to_string_with_context(&mut context)
    }

    fn to_string_with_context(&self, context: &mut ExpressionContext) -> String {
        match self {
            ConditionalExpression::Bracket(expression) => {
                format!("({})", expression.to_string_with_context(context))
            }
            ConditionalExpression::Unit {
                key,
                relational_operation,
            } => format!(
                "{} {}",
                key.as_variable(),
                relational_operation.to_string(context)
            ),
            ConditionalExpression::Binary {
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

#[test]
fn test_conditional_expression_generation() {
    use crate::dynamodb_context::expression::conditional::expression_builder::BuildConditionalExpression;
    let pk = String::from("pk");
    let sk = String::from("sk");

    let expression = pk
        .string_equals("User")
        .and()
        .expr(sk.string_equals("user123"));

    assert_eq!("#var_pk = :v1 AND #var_sk = :v2", expression.to_string());

    let attribute_values = expression.get_expression_attribute_values();
    assert!(attribute_values.len() == 2);
    assert!({
        attribute_values.get(":vc1").unwrap().as_s().unwrap() == "User"
            && attribute_values.get(":vc2").unwrap().as_s().unwrap() == "user123"
    });

    let attribute_names = expression.get_expression_attribute_names();
    assert!(attribute_names.len() == 2);
    assert!({
        attribute_names.get("#var_pk").unwrap() == "pk"
            && attribute_names.get("#var_sk").unwrap() == "sk"
    })
}
