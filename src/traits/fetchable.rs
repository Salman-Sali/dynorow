use std::collections::HashMap;

use aws_sdk_dynamodb::{operation::get_item::GetItemOutput, types::AttributeValue};

use super::as_projection::AsProjection;


pub trait Fetchable : TryFrom<HashMap<String, AttributeValue>, Error = <Self as Fetchable>::Error> + TryFrom<GetItemOutput, Error = <Self as Fetchable>::Error> + Clone + std::fmt::Debug + AsProjection {
    type Error: std::fmt::Debug;
}