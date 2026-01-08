use crate::key::KeyValue;

pub trait AsPkAvailableCompositeKeyValue {
    fn as_pk_available_composite_key_value(sort_key_value: String) -> KeyValue;
}

pub trait AsCompositeKeyValue {
    fn as_composite_key_value(partition_key_value: String, sort_key_value: String) -> KeyValue;
}

pub trait AsPartitionKeyValue {
    fn as_partition_key_value(partition_key_value: String) -> KeyValue;
}

pub trait AsValueAvailablePkValue {
    fn as_value_available_pk_value() -> KeyValue;
}

pub trait AsKeyValue {
    fn as_key_value(&self) -> KeyValue;
}