use aws_sdk_dynamodb::operation::get_item::GetItemOutput;

pub trait HasValue {
    fn has_value(&self) -> bool;
}

impl HasValue for GetItemOutput {
    fn has_value(&self) -> bool {
        self.item.is_some()
    }
}