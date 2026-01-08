use aws_sdk_dynamodb::
    operation::put_item::builders::PutItemFluentBuilder
;

use crate::traits::as_attribute_key_values::AsAttributeKeyValues;

pub trait ItemsFrom<T>
where
    T: AsAttributeKeyValues,
{
    fn items_from(self, t: &T) -> Self;
}

impl<T> ItemsFrom<T> for PutItemFluentBuilder
where
    T: AsAttributeKeyValues,
{
    fn items_from(mut self, t: &T) -> Self {
        let attribute_key_values = t.as_attribute_key_values();
        for x in attribute_key_values {
            self = self.item(x.0, x.1);
        }
        self
    }
}