use std::collections::HashMap;

use crate::dynamodb_context::expression::{AsVariable, ExpressionContext};
use aws_sdk_dynamodb::types::AttributeValue;

pub mod expression_builder;

#[derive(Debug, Clone)]
pub struct UpdateExpression {
    sets: Vec<SetOperation>,
    adds: Vec<AddOperation>,
    /// Removes the matching attribute from row.
    /// Should only use this on Option<T>
    removes: Vec<String>,
    /// Deletes matching value from list
    deletes: Vec<DeleteOperation>,
}

impl UpdateExpression {
    pub fn new_set(set: SetOperation) -> UpdateExpression {
        UpdateExpression {
            sets: vec![set],
            adds: vec![],
            removes: vec![],
            deletes: vec![],
        }
    }

    pub fn new_add(key: String, value: AttributeValue) -> UpdateExpression {
        UpdateExpression {
            sets: vec![],
            adds: vec![AddOperation { key, value }],
            removes: vec![],
            deletes: vec![],
        }
    }

    pub fn new_remove(remove: String) -> UpdateExpression {
        UpdateExpression {
            sets: vec![],
            adds: vec![],
            removes: vec![remove],
            deletes: vec![],
        }
    }

    pub fn new_delete(key: String, value: AttributeValue) -> UpdateExpression {
        UpdateExpression {
            sets: vec![],
            adds: vec![],
            removes: vec![],
            deletes: vec![DeleteOperation { key, value }],
        }
    }

    pub fn and(mut self, mut exp: UpdateExpression) -> Self {
        self.sets.append(&mut exp.sets);
        self.adds.append(&mut exp.adds);
        self.removes.append(&mut exp.removes);
        self.deletes.append(&mut exp.deletes);
        self
    }

    pub fn to_string(&self) -> String {
        let mut context = ExpressionContext::new("vu");
        self.to_string_with_context(&mut context)
    }

    fn to_string_with_context(&self, context: &mut ExpressionContext) -> String {
        let mut result = String::new();

        if !self.sets.is_empty() {
            result += "\n";
            result += "SET ";
            result += &self
                .sets
                .iter()
                .map(|x| x.to_string(context))
                .collect::<Vec<String>>()
                .join(", ");
        }

        if !self.adds.is_empty() {
            result += "\n";
            result += "ADD ";
            result += &self
                .adds
                .iter()
                .map(|x| x.to_string(context))
                .collect::<Vec<String>>()
                .join(", ");
        }

        if !self.removes.is_empty() {
            result += "\n";
            result += "REMOVE ";
            result += &self.removes.join(", ");
        }

        if !self.deletes.is_empty() {
            result += "\n";
            result += "DELETE ";
            result += &self
                .deletes
                .iter()
                .map(|x| x.to_string(context))
                .collect::<Vec<String>>()
                .join(", ")
        }

        result
    }

    pub fn get_expression_attribute_names(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();

        self.sets.iter().for_each(|x| match x {
            SetOperation::Assign { key, value: _ }
            | SetOperation::Increment { key, value: _ }
            | SetOperation::Decrement { key, value: _ }
            | SetOperation::IfNotExists { key, value: _ }
            | SetOperation::ListAppend { key, value: _ }
            | SetOperation::ListPrepend { key, value: _ } => {
                result.insert(key.as_variable(), key.clone());
            }
        });

        self.adds.iter().for_each(|x| {
            result.insert(x.key.as_variable(), x.key.clone());
        });

        self.removes.iter().for_each(|x| {
            result.insert(x.as_variable(), x.clone());
        });

        self.deletes.iter().for_each(|x| {
            result.insert(x.key.as_variable(), x.key.clone());
        });

        return result;
    }

    pub fn get_expression_attribute_values(&self) -> HashMap<String, AttributeValue> {
        let mut context = ExpressionContext::new("vu");
        let mut result: HashMap<String, AttributeValue> = HashMap::new();

        self.sets.iter().for_each(|x| match x {
            SetOperation::Assign { key: _, value }
            | SetOperation::Increment { key: _, value }
            | SetOperation::Decrement { key: _, value }
            | SetOperation::IfNotExists { key: _, value }
            | SetOperation::ListAppend { key: _, value }
            | SetOperation::ListPrepend { key: _, value } => {
                result.insert(context.next(), value.clone());
            }
        });

        self.adds.iter().for_each(|x| {
            result.insert(context.next(), x.value.clone());
        });

        self.deletes.iter().for_each(|x| {
            result.insert(context.next(), x.value.clone());
        });

        return result;
    }
}

#[derive(Debug, Clone)]
pub enum SetOperation {
    Assign { key: String, value: AttributeValue },
    Increment { key: String, value: AttributeValue },
    Decrement { key: String, value: AttributeValue },
    IfNotExists { key: String, value: AttributeValue },
    ListAppend { key: String, value: AttributeValue },
    ListPrepend { key: String, value: AttributeValue },
}

impl SetOperation {
    pub fn to_string(&self, context: &mut ExpressionContext) -> String {
        let variable = context.next();
        match self {
            SetOperation::Assign { key, value: _ } => {
                format!("{} = {}", key.as_variable(), variable)
            }
            SetOperation::Increment { key, value: _ } => {
                format!(
                    "{} = {} + {}",
                    key.as_variable(),
                    key.as_variable(),
                    variable
                )
            }
            SetOperation::Decrement { key, value: _ } => {
                format!(
                    "{} = {} - {}",
                    key.as_variable(),
                    key.as_variable(),
                    variable
                )
            }
            SetOperation::IfNotExists { key, value: _ } => {
                format!(
                    "{} = if_not_exists({}, {})",
                    key.as_variable(),
                    key.as_variable(),
                    variable
                )
            }
            SetOperation::ListAppend { key, value: _ } => {
                format!(
                    "{} = list_append({}, {})",
                    key.as_variable(),
                    key.as_variable(),
                    variable
                )
            }
            SetOperation::ListPrepend { key, value: _ } => {
                format!(
                    "{} = list_append({}, {})",
                    key.as_variable(),
                    key.as_variable(),
                    variable
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddOperation {
    pub key: String,
    pub value: AttributeValue,
}

impl AddOperation {
    pub fn to_string(&self, context: &mut ExpressionContext) -> String {
        format!("{} {}", self.key.as_variable(), context.next())
    }
}

#[derive(Debug, Clone)]
pub struct DeleteOperation {
    pub key: String,
    pub value: AttributeValue,
}

impl DeleteOperation {
    pub fn to_string(&self, context: &mut ExpressionContext) -> String {
        format!("{} {}", self.key.as_variable(), context.next())
    }
}

#[cfg(test)]
pub mod test_update_expression_to_string {
    use std::collections::HashSet;

    use aws_sdk_dynamodb::types::AttributeValue;

    use crate::{
        UpdateExpression, dynamodb_context::expression::update::SetOperation,
        traits::into_attribute_value::IntoAttributeValue,
    };

    #[test]
    fn test_sets() {
        let expression = UpdateExpression::new_set(SetOperation::Assign {
            key: "user".into(),
            value: AttributeValue::S("Username1".into()),
        })
        .and(UpdateExpression::new_set(SetOperation::Increment {
            key: "total_sales".into(),
            value: AttributeValue::N(1.to_string()),
        }))
        .and(UpdateExpression::new_set(SetOperation::IfNotExists {
            key: "DeactivatedOn".into(),
            value: AttributeValue::S("172432342".into()),
        }));

        assert_eq!(
            expression.to_string(),
            "\nSET #var_user = :vu1, #var_total_sales = #var_total_sales + :vu2, #var_DeactivatedOn = if_not_exists(#var_DeactivatedOn, :vu3)"
        );
    }

    #[test]
    fn test_adds() {
        let expression =
            UpdateExpression::new_add("count".into(), AttributeValue::N(1.to_string())).and(
                UpdateExpression::new_add("other_count".into(), AttributeValue::N(format!("-1"))),
            );

        assert_eq!(
            expression.to_string(),
            "\nADD #var_count :vu1, #var_other_count :vu2"
        );
    }

    #[test]
    fn test_removes() {
        let expression = UpdateExpression::new_remove("disabled_on".into())
            .and(UpdateExpression::new_remove("deleted_on".into()));

        assert_eq!(expression.to_string(), "\nREMOVE disabled_on, deleted_on");
    }

    #[test]
    fn test_deletes() {
        let mut ids_to_remove: HashSet<String> = HashSet::new();
        ids_to_remove.insert("abc123".into());
        let expression =
            UpdateExpression::new_delete("valid_ids".into(), ids_to_remove.into_attribute_value());

        assert_eq!(expression.to_string(), "\nDELETE #var_valid_ids :vu1");
    }

    #[test]
    fn test_multiple() {
        let mut ids_to_remove: HashSet<String> = HashSet::new();
        ids_to_remove.insert("abc123".into());
        let expression = UpdateExpression::new_set(SetOperation::Assign {
            key: "user".into(),
            value: AttributeValue::S("Username1".into()),
        })
        .and(UpdateExpression::new_set(SetOperation::Increment {
            key: "total_sales".into(),
            value: AttributeValue::N(1.to_string()),
        }))
        .and(UpdateExpression::new_set(SetOperation::IfNotExists {
            key: "DeactivatedOn".into(),
            value: AttributeValue::S("172432342".into()),
        }))
        .and(UpdateExpression::new_add(
            "count".into(),
            AttributeValue::N(1.to_string()),
        ))
        .and(UpdateExpression::new_add(
            "other_count".into(),
            AttributeValue::N(format!("-1")),
        ))
        .and(UpdateExpression::new_remove("disabled_on".into()))
        .and(UpdateExpression::new_remove("deleted_on".into()))
        .and(UpdateExpression::new_delete(
            "valid_ids".into(),
            ids_to_remove.into_attribute_value(),
        ));
        assert_eq!(
            expression.to_string(),
            r#"
SET #var_user = :vu1, #var_total_sales = #var_total_sales + :vu2, #var_DeactivatedOn = if_not_exists(#var_DeactivatedOn, :vu3)
ADD #var_count :vu4, #var_other_count :vu5
REMOVE disabled_on, deleted_on
DELETE #var_valid_ids :vu6"#
        );
    }
}
