use aws_sdk_dynamodb::operation::get_item::builders::GetItemFluentBuilder;

use crate::traits::as_projection::AsProjection;

pub trait ProjectedAs {
    fn projected_as<T : AsProjection>(self) -> GetItemFluentBuilder;
}

impl ProjectedAs for GetItemFluentBuilder {
    fn projected_as<T : AsProjection>(self) -> GetItemFluentBuilder {
        self.projection_expression(T::as_projection())
    }
}