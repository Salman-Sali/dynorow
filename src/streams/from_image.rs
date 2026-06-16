use std::collections::HashMap;

use serde_dynamo::Item;

use crate::traits::serde_dynamo_attribute_value_into::SerdeDynamoAttributeValueHashMapInto;

#[async_trait::async_trait]
pub trait FromImage<T> {
    fn from_image(image: Item) -> Result<T, ()>;
}

#[async_trait::async_trait]
impl<T, E> FromImage<T> for T
where
    T: TryFrom<HashMap<String, crate::aws_sdk_dynamodb::types::AttributeValue>, Error = E> + Send,
    E: std::fmt::Debug,
{
    fn from_image(image: Item) -> Result<T, ()> {
        Self::try_from(image.into_inner().into_aws_attribute_value_hashmap()).map_err(|e| {
            eprintln!("Error while converting from item HashMap : {:?}", e);
            ()
        })
    }
}
