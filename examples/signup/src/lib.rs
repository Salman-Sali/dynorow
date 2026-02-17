use std::collections::HashSet;

use chrono::{DateTime, Utc};
use dynorow::{
    BuildConditionalExpression, DynoMap, DynoRow, Fetchable, Insertable, Updatable,
    traits::matches_template::MatchesTemplate,
};

pub fn get_table_name() -> String {
    String::from("my_table_name")
}

#[derive(Default, DynoRow, Insertable, Fetchable, Updatable, Clone, Debug)]
#[dynorow(table =  get_table_name())]
#[dynorow(pk = "pk")]
#[dynorow(pk_value = "signup")]
pub struct SignUp {
    #[dynorow(sk)]
    #[dynorow(key = "sk")]
    pub email_address: String,

    #[dynorow(key = "retry")]
    pub retry_count: i32,

    #[dynorow(ignore)]
    pub somthing: u32,

    pub uid: String,
    pub password: Option<String>,
    pub data: Option<Data>,
    pub string_set: HashSet<String>,

    #[dynorow(serde)]
    pub deleted_on: Option<DateTime<Utc>>,
}

#[derive(Clone, Default, Debug, DynoMap)]
pub struct Data {
    pub something: i32,
    pub list_of_items: Vec<String>,
}

#[derive(DynoRow, Clone, Debug, Fetchable)]
#[dynorow(pk = "pk")]
#[dynorow(pk_value = "SaleConfirmed:{email_address}:{sale_id}")]
pub struct SaleConfirmed {
    #[dynorow(sk)]
    #[dynorow(key = "sk")]
    pub order_id: String,
    pub email_address: String,
    pub sale_id: String,
}

pub async fn insert() {
    let config = aws_config::from_env().load().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    let context = dynorow::DynamodbContext::new(client);
    let _ = context.insert_row(SignUp::default()).await;
    let _ = context.insert_row(SignUp::default());

    let _ = context
        .with_table("RandomTableName") //you can use with_table as an alternative to #[dynorow(table = "table_name")]
        .get::<SaleConfirmed>(SaleConfirmed::generate_composite_key(
            "myemail@email.com",
            "sales_123",
            "order_1234",
        ))
        .await;

    let update_expression = SignUp::update_expression_builder()
        .data_fields()
        .something()
        .add_decrement(1);

    let condition = SignUp::conditional_expression_builder()
        .retry_count()
        .equals(5);

    let _ = context
        .update_with_condition::<SignUp>(
            SignUp::generate_composite_key("my_email_address"),
            update_expression,
            condition,
        )
        .await;

    assert!(SaleConfirmed::matches_template(
        "SaleConfirmed:email@somthing.com:sale_123"
    ));
}

#[test]
pub fn test_update_expression() {
    let update_expression = SignUp::update_expression_builder()
        .data_fields()
        .something()
        .add_decrement(1);

    assert_eq!(
        "\nADD #var_data_something :vu1",
        update_expression.to_string()
    );

    assert_eq!(
        format!("{:?}", update_expression.get_expression_attribute_names()),
        r##"{"#var_data_something": "data.something"}"##
    );
    assert_eq!(
        format!("{:?}", update_expression.get_expression_attribute_values()),
        r#"{":vu1": N("-1")}"#
    )
}
