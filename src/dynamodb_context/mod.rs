mod dynamodb_table;
pub mod expression;
pub mod get_result_list;
pub mod operations;

use dynamodb_table::DynamodbTable;

use crate::{
    error::Error, key::KeyValue, traits::{
        as_key_value::AsPkAvailableCompositeKeyValue, fetchable::Fetchable, has_key::HasKey,
        has_pk_value::HasPkValue, has_table_name::HasTableName, insertable::Insertable,
    }, Expression, GetListResult
};

#[derive(Debug)]
pub struct DynamodbContext {
    pub client: aws_sdk_dynamodb::Client,
}

impl DynamodbContext {
    pub fn new(client: aws_sdk_dynamodb::Client) -> Self {
        Self { client }
    }

    pub fn with_table(&'_ self, table_name: &str) -> DynamodbTable<'_> {
        DynamodbTable::new(table_name.to_string(), &self.client)
    }

    pub async fn exists<T: Fetchable + HasTableName>(
        &self,
        key: crate::key::KeyValue,
    ) -> Result<bool, crate::error::Error> {
        self.with_table(&T::get_table_name()).exists::<T>(key).await
    }

    pub async fn exists_with_sort_key<
        T: Fetchable + HasTableName + AsPkAvailableCompositeKeyValue,
    >(
        &self,
        sort_key_value: String,
    ) -> Result<bool, crate::error::Error> {
        self.with_table(&T::get_table_name())
            .exists_with_sort_key::<T>(sort_key_value)
            .await
    }

    pub async fn get<T: Fetchable + HasTableName>(
        &self,
        key: crate::key::KeyValue,
    ) -> Result<T, crate::error::Error> {
        self.with_table(&T::get_table_name()).get(key).await
    }

    pub async fn get_with_sort_key<T: Fetchable + AsPkAvailableCompositeKeyValue + HasTableName>(
        &self,
        sort_key_value: String,
    ) -> Result<T, crate::error::Error> {
        self.with_table(&T::get_table_name())
            .get_with_sort_key(sort_key_value)
            .await
    }

    pub async fn get_maybe<T: Fetchable + HasTableName>(
        &self,
        key: crate::key::KeyValue,
    ) -> Result<Option<T>, crate::error::Error> {
        self.with_table(&T::get_table_name()).get_maybe(key).await
    }

    pub async fn get_maybe_with_sort_key<
        T: Fetchable + AsPkAvailableCompositeKeyValue + HasTableName,
    >(
        &self,
        sort_key_value: String,
    ) -> Result<Option<T>, crate::error::Error> {
        self.with_table(&T::get_table_name())
            .get_maybe_with_sort_key(sort_key_value)
            .await
    }

    pub async fn insert_row<T: Insertable + HasTableName>(
        &self,
        row: T,
    ) -> Result<(), crate::error::Error> {
        self.with_table(&T::get_table_name()).insert_row(row).await
    }

    pub async fn update<T: crate::traits::updatable::Updatable + HasTableName>(
        &self,
        row: T,
    ) -> Result<(), crate::error::Error> {
        self.with_table(&T::get_table_name()).update(row).await
    }

    pub async fn delete<T: HasTableName>(&self, key_value: KeyValue) -> Result<(), Error> {
        self.with_table(&T::get_table_name()).delete(key_value).await
    }

    pub async fn delete_with_sort_key<T : AsPkAvailableCompositeKeyValue + HasTableName>(&self, sort_key_value: String) -> Result<(), Error> {
        self.with_table(&T::get_table_name()).delete_with_sort_key::<T>(sort_key_value).await
    }

    pub async fn delete_with_condition<T: HasTableName>(
        &self,
        key_value: KeyValue,
        conditional_expression: Option<Expression>,
    ) -> Result<(), Error> {
        self.with_table(&T::get_table_name()).delete_with_condition(key_value, conditional_expression).await
    }

    pub async fn get_list_with_key_value<T: Fetchable + HasKey + HasTableName>(
        &self,
        key_value: KeyValue,
        count: u16,
        last_key_value: Option<KeyValue>,
        accending: bool
    ) -> Result<GetListResult<T>, Error> {
        self.with_table(&T::get_table_name())
            .get_list_with_key_value(key_value, count, last_key_value, accending)
            .await
    }

    pub async fn get_list<T: Fetchable + HasKey + HasPkValue + HasTableName>(
        &self,
        count: u16,
        last_key_value: Option<KeyValue>,
        accending: bool
    ) -> Result<GetListResult<T>, Error> {
        self.with_table(&T::get_table_name())
            .get_list(count, last_key_value, accending)
            .await
    }

    pub async fn get_list_with_condition<T: Fetchable + HasKey + HasTableName>(
        &self,
        conditional_expression: Expression,
        count: u16,
        last_key_value: Option<KeyValue>,
        accending: bool
    ) -> Result<GetListResult<T>, Error> {
        self.with_table(&T::get_table_name())
            .get_list_with_condition(conditional_expression, count, last_key_value, accending)
            .await
    }

    
}