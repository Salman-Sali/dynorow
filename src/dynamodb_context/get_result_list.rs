use crate::key::KeyValue;

pub struct GetListResult<T> {
    pub items: Vec<T>,
    pub last_key_value: Option<KeyValue>,
}

impl<T> GetListResult<T> {
    pub fn new(items: Vec<T>, last_key: Option<KeyValue>) -> Self {
        Self { items, last_key_value: last_key }
    }
}
