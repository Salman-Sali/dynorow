use crate::{key::KeyValue, traits::insertable::Insertable};

pub enum Operation {
    Insert(Box<dyn Insertable>),
    Delete(KeyValue),
}

impl Operation {
    pub fn new_insert<T: Insertable>(item: T) -> Operation {
        Operation::Insert(Box::new(item))
    }
}
