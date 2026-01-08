use dynorow::{
    DynoRow, Fetchable, Insertable, traits::as_key_value::AsPkAvailableCompositeKeyValue,
};

#[derive(Default, DynoRow, Insertable, Fetchable, Clone, Debug)]
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
    pub password: String,
}

pub fn get_table_name() -> String {
    String::from("my_table_name")
}

pub async fn insert() {
    let config = aws_config::from_env().load().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    let context = dynorow::DynamodbContext::new(client);
    let _ = context.insert_row(SignUp::default()).await;
    let _ = context
        .with_table("my_other_table")
        .insert_row(SignUp::default());

    let sign_up = context
        .get::<SignUp>(SignUp::as_pk_available_composite_key_value(
            "myemail@email.com".into(),
        ))
        .await;
}
