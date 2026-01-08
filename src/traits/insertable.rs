use super::as_attribute_key_values::AsAttributeKeyValues;

pub trait Insertable: AsAttributeKeyValues + 'static + Send {}