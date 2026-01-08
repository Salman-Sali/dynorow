use super::{as_attribute_key_values::AsAttributeKeyValues, as_key_value::AsKeyValue};

pub trait Updatable :  AsAttributeKeyValues + 'static + std::fmt::Debug + AsKeyValue {
    
}