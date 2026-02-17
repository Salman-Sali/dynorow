use std::collections::HashMap;

use aws_sdk_dynamodb::{
    operation::update_item::builders::UpdateItemFluentBuilder, types::AttributeValue,
};

use crate::{
    key::KeyValue,
    traits::{as_attribute_key_values::AsAttributeKeyValues, as_key_value::AsKeyValue},
};

pub trait WithExpression {
    fn with_expression<T: AsAttributeKeyValues + AsKeyValue>(self, row: &T) -> Self;
}

impl WithExpression for UpdateItemFluentBuilder {
    fn with_expression<T: AsAttributeKeyValues + AsKeyValue>(self, row: &T) -> Self {
        let variable_value_map = generate_expression_variable_value_map(row);
        let update_expression = generate_update_expression(&variable_value_map);

        let mut builder = self.update_expression(update_expression);
        builder = generate_expression_attribute_names(builder, &variable_value_map);
        builder = generate_expression_attribute_values(builder, &variable_value_map);
        builder
    }
}

fn generate_expression_attribute_values(
    mut builder: UpdateItemFluentBuilder,
    variable_value_map: &HashMap<String, (String, AttributeValue)>,
) -> UpdateItemFluentBuilder {
    for variable_value in variable_value_map {
        builder = builder.expression_attribute_values(variable_value.0, variable_value.1.1.clone());
    }
    builder
}

fn generate_expression_attribute_names(
    mut builder: UpdateItemFluentBuilder,
    variable_value_map: &HashMap<String, (String, AttributeValue)>,
) -> UpdateItemFluentBuilder {
    for variable_value in variable_value_map {
        builder = builder.expression_attribute_names(
            format!("#{}", variable_value.1.0),
            variable_value.1.0.clone(),
        );
    }
    builder
}

fn generate_update_expression(
    variable_value_map: &HashMap<String, (String, AttributeValue)>,
) -> String {
    const SET: &str = "SET";
    let mut result = String::from(SET);
    for variable_value in variable_value_map {
        if &result != SET {
            result += ",";
        }
        result += &format!(" #{} = {}", variable_value.1.0, variable_value.0);
    }
    result
}

fn generate_expression_variable_value_map<T: AsAttributeKeyValues + AsKeyValue>(
    row: &T,
) -> HashMap<String, (String, AttributeValue)> {
    let mut result: HashMap<String, (String, AttributeValue)> = HashMap::new();

    let key = row.as_key_value();

    for attribute_key_value in row.as_attribute_key_values() {
        if !is_key(&key, &attribute_key_value.0) {
            result.insert(as_variable(&attribute_key_value.0), attribute_key_value);
        }
    }

    return result;
}

fn as_variable(key: &String) -> String {
    format!(":v_{}", key)
}

fn is_key(key: &KeyValue, value_key: &String) -> bool {
    match key {
        KeyValue::CompositeKey {
            partition_key,
            partition_key_value: _,
            sort_key,
            sort_key_value: _,
        } => partition_key == value_key || sort_key == value_key,
        KeyValue::PartitionKey { key, value: _ } => key == value_key,
    }
}
