use aws_sdk_dynamodb::operation::get_item::GetItemOutput;

pub trait TryIntoRow {
    fn try_into_row<T: TryFrom<GetItemOutput>>(self) -> Result<T, T::Error>;
}

impl TryIntoRow for GetItemOutput {
    fn try_into_row<T: TryFrom<GetItemOutput>>(self) -> Result<T, T::Error> {
        T::try_from(self)
    }
}