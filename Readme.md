### Dynorow invites you to a new way of designing your dynamodb models in rust.

This is a work in progress project. I am using this at production but you should use at your own risk. Right now I am pushing features based on my need. Please feel free to raise a PR.

dynorow is a Rust library that provides strongly typed, derive-based access to Amazon DynamoDB.
It focuses on simple, compile-time-checked models with expressive update and conditional expression builders.

Conditional expression builders and filter expressions are very very WIP. I wouldn't use those now.

## Features
- Derive macros for table models
- Typed insert, fetch, and update operations
- Compile-time safe key templates
- Expression builders for updates and conditions
- Nested map support
- Optional serde-based fields
- Minimal boilerplate

## Upcoming Maybe
- Streams Handlers: I already have a streams handler running for my production project based on dynorow, however it is not release ready. If I see enough intreset for it, maybe I will consider cleaning it up and publishing it.


## Installation 
```bash
cargo add dynorow
```

## Basic Example
```rust
use std::collections::HashSet;

use chrono::{DateTime, Utc};
use dynorow::{
    BuildConditionalExpression, DynoMap, DynoRow, Fetchable, Insertable, Updatable,
    traits::{as_key_value::AsPkAvailableCompositeKeyValue, matches_template::MatchesTemplate},
};

pub fn get_table_name() -> String {
    String::from("my_table_name")
}

#[derive(Default, DynoRow, Insertable, Fetchable, Updatable, Clone, Debug)]
#[dynorow(table =  get_table_name())]
#[dynorow(pk = "pk")]
#[dynorow(pk_value = "User:SignUp")]
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
```


## Composite Key Templates
```rust
#[derive(DynoRow, Clone, Debug)]
#[dynorow(pk = "pk")]
#[dynorow(pk_value = "SaleConfirmed:{email_address}:{sale_id}")]
pub struct SaleConfirmed {
    #[dynorow(sk)]
    #[dynorow(key = "sk")]
    pub order_id: String,
    pub email_address: String,
    pub sale_id: String,
}
```

You can also perform `matches_template` on a string pk value to verify if it belongs to this model
```rust
assert!(SaleConfirmed::matches_template(
    "SaleConfirmed:email@somthing.com:sale_123"
));
```


## Some Sample Code
```rust 
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
```

## Derive Macros
### DynoRow

Defines a DynamoDB-backed struct.

Attributes:

- `#[dynorow(table = ...)]` – table name
- `#[dynorow(pk = "...")]` – partition key attribute name
- `#[dynorow(pk_value = "...")]` – static or templated PK value

Field attributes:

- `#[dynorow(sk)]` – marks sort key field
- `#[dynorow(key = "...")]` – custom attribute name
- `#[dynorow(ignore)]` – excluded from DynamoDB
- `#[dynorow(serde)]` – stored using serde

<br>

### DynoMap

Marks nested structs that map to DynamoDB map attributes.

Operation Traits

- Insertable – enables inserts
- Fetchable – enables gets
- Updatable – enables updates

### Philosophy
- Strong typing over stringly-typed queries
- Compile-time guarantees where possible
- Simple, explicit models
- No dealing with low level aws sdk where ever possible


Dynamodb is a very capable database. I use dynamodb for its low cost when the usage is low or idle, plus scalability when I need it. Dynorow will help you keep you at your business logic level without pulling you down into low level aws sdk, where maintaing and enforcing your business rules can be very cumbersome.
