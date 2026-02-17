use crate::{
    GetListResult, UpdateExpression,
    dynamodb_context::expression::conditional::{
        ConditionalExpression, expression_builder::BuildConditionalExpression,
    },
    dynamodb_sdk_extensions::{
        items_from::ItemsFrom, with_expresssion::WithExpression, with_key::WithKey,
    },
    error::Error,
    key::KeyValue,
    traits::{
        as_key_value::AsPkAvailableCompositeKeyValue, fetchable::Fetchable, has_key::HasKey,
        has_pk_value::HasStaticPkValue, insertable::Insertable, updatable::Updatable,
    },
};

use futures::future::join_all;
use rand::Rng;
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;

use async_recursion::async_recursion;
use aws_sdk_dynamodb::types::{DeleteRequest, PutRequest, WriteRequest};

use super::operations::Operation;

pub struct DynamodbTable<'a> {
    pub table_name: String,
    pub client: &'a aws_sdk_dynamodb::Client,
}

impl<'a> DynamodbTable<'a> {
    pub fn new(table_name: String, client: &'a aws_sdk_dynamodb::Client) -> Self {
        Self { table_name, client }
    }

    pub async fn exists_with_sort_key<T: Fetchable + AsPkAvailableCompositeKeyValue>(
        &self,
        sort_key_value: String,
    ) -> Result<bool, Error> {
        let key = T::as_pk_available_composite_key_value(sort_key_value);
        self.exists::<T>(key).await
    }

    pub async fn exists<T: Fetchable>(&self, key: KeyValue) -> Result<bool, Error> {
        let row_exists_result = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .with_key(&key)
            .projection_expression(key.project_key())
            .send()
            .await;

        return match row_exists_result {
            Ok(x) => Ok(x.item.is_some()),
            Err(e) => Err(Error::sdk_error("Failure while checking if row exists", e)),
        };
    }

    pub async fn get<T: Fetchable>(&self, key: KeyValue) -> Result<T, Error> {
        let get_item_output = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .with_key(&key)
            .projection_expression(T::as_projection())
            .set_expression_attribute_names(Some(T::as_projection_names()))
            .send()
            .await
            .map_err(|e| Error::sdk_error("Get item from dynamodb failed.", e))?;

        T::try_from(get_item_output.clone()).map_err(|e| {
            eprintln!(
                "Error: {:?}, Data: {:?}, Key: {:?}",
                e, get_item_output, key
            );
            Error::sdk_error(
                "Failure while converting dyanmodb result into row",
                format!("{:?}", e),
            )
        })
    }

    pub async fn get_with_sort_key<T: Fetchable + AsPkAvailableCompositeKeyValue>(
        &self,
        sort_key_value: String,
    ) -> Result<T, Error> {
        let key = T::as_pk_available_composite_key_value(sort_key_value);
        self.get(key).await
    }

    pub async fn get_maybe<T: Fetchable>(&self, key: KeyValue) -> Result<Option<T>, Error> {
        let get_item_output = self
            .client
            .get_item()
            .table_name(self.table_name.clone())
            .with_key(&key)
            .projection_expression(T::as_projection())
            .set_expression_attribute_names(Some(T::as_projection_names()))
            .send()
            .await
            .map_err(|e| Error::sdk_error("Get item from dynamodb failed.", e))?;

        if get_item_output.item().is_none() {
            return Ok(None);
        };

        T::try_from(get_item_output.clone())
            .map(|x| Some(x))
            .map_err(|e| {
                eprintln!(
                    "Error: {:?}, Data: {:?}, Key: {:?}",
                    e, get_item_output, key
                );
                Error::sdk_error(
                    "Failure while converting dyanmodb result into row",
                    format!("{:?}", e),
                )
            })
    }

    pub async fn get_maybe_with_sort_key<T: Fetchable + AsPkAvailableCompositeKeyValue>(
        &self,
        sort_key_value: String,
    ) -> Result<Option<T>, Error> {
        let key = T::as_pk_available_composite_key_value(sort_key_value);
        self.get_maybe(key).await
    }

    pub async fn get_list_with_pk_value<T: Fetchable + HasKey>(
        &self,
        pk_value: KeyValue,
        count: u16,
        last_key_value: Option<KeyValue>,
        accending: bool,
    ) -> Result<GetListResult<T>, Error> {
        let key_value = pk_value.into_partition_key_value();
        let expression = key_value.into_conditional_expression();
        self.get_list_with_condition(expression, count, last_key_value, accending)
            .await
    }

    pub async fn get_list<T: Fetchable + HasKey + HasStaticPkValue>(
        &self,
        count: u16,
        last_key_value: Option<KeyValue>,
        accending: bool,
    ) -> Result<GetListResult<T>, Error> {
        let key_expression = T::get_key()
            .get_partition_key()
            .string_equals(&T::get_static_pk_value());

        self.get_list_with_condition(key_expression, count, last_key_value, accending)
            .await
    }

    pub async fn get_list_with_condition<T: Fetchable + HasKey>(
        &self,
        key_conditional_expression: ConditionalExpression,
        count: u16,
        last_key_value: Option<KeyValue>,
        accending: bool,
    ) -> Result<GetListResult<T>, Error> {
        let mut query = self
            .client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression(key_conditional_expression.to_string())
            .set_expression_attribute_names(Some(
                key_conditional_expression.get_expression_attribute_names(),
            ))
            .set_expression_attribute_values(Some(
                key_conditional_expression.get_expression_attribute_values(),
            ))
            .key_condition_expression(key_conditional_expression.to_string())
            .set_expression_attribute_names(Some(
                key_conditional_expression.get_expression_attribute_names(),
            ))
            .set_expression_attribute_values(Some(
                key_conditional_expression.get_expression_attribute_values(),
            ))
            .scan_index_forward(accending)
            .limit(count as i32);

        if let Some(last_key) = last_key_value.clone() {
            query = query.set_exclusive_start_key(Some(last_key.into_hash_map()));
        }

        let query_result = query.send().await.map_err(|e| {
            eprintln!("Error: {:?}, Key: {:?}", e, last_key_value);
            Error::sdk_error("Error while performaing get list query.", e)
        })?;

        let mut result: Vec<T> = vec![];

        if let Some(items) = query_result.items {
            for item in items {
                let t = T::try_from(item).map_err(|e| {
                    Error::sdk_error(
                        "Error while converting Hashmap<Stirng, AttributeValue> into T.",
                        format!("{:?}", e),
                    )
                })?;
                result.push(t);
            }
        }

        let last_key = if let Some(x) = query_result.last_evaluated_key {
            Some(KeyValue::from_hash_map(x, T::get_key())?)
        } else {
            None
        };

        return Ok(GetListResult::new(result, last_key));
    }

    pub async fn insert_row<T: Insertable>(&self, row: T) -> Result<(), Error> {
        self.client
            .put_item()
            .table_name(self.table_name.clone())
            .items_from(&row)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| {
                eprintln!("Error: {:?}", e);
                Error::sdk_error("Put item into dynamodb failed.", e)
            })
    }

    pub async fn update<T: Updatable>(&self, row: T) -> Result<(), Error> {
        self.client
            .update_item()
            .table_name(self.table_name.clone())
            .with_key(&row.as_key_value())
            .with_expression(&row)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| {
                eprintln!("Error : {:?}, Data: {:?}", e, row);
                Error::sdk_error("Update item into dynamodb failed.", e)
            })
    }

    pub async fn update_with_expression<T: Updatable>(
        &self,
        key_value: KeyValue,
        expression: UpdateExpression,
    ) -> Result<(), Error> {
        self.client
            .update_item()
            .table_name(&self.table_name)
            .set_key(Some(key_value.clone().into_hash_map()))
            .update_expression(expression.to_string())
            .set_expression_attribute_names(Some(expression.get_expression_attribute_names()))
            .set_expression_attribute_values(Some(expression.get_expression_attribute_values()))
            .send()
            .await
            .map(|_| ())
            .map_err(|e| {
                eprintln!(
                    "Error : {:?},  Key: {:?} update expression: {:?}",
                    e, key_value, expression
                );
                Error::sdk_error("Dynamodb delete item failed.", e)
            })
    }

    pub async fn update_with_condition<T: Updatable>(
        &self,
        key_value: KeyValue,
        update: UpdateExpression,
        condition: ConditionalExpression,
    ) -> Result<(), Error> {
        let mut attribute_names = update.get_expression_attribute_names();
        attribute_names.extend(condition.get_expression_attribute_names());

        let mut attribute_values = update.get_expression_attribute_values();
        attribute_values.extend(condition.get_expression_attribute_values());

        self.client
            .update_item()
            .table_name(&self.table_name)
            .set_key(Some(key_value.clone().into_hash_map()))
            .update_expression(update.to_string())
            .condition_expression(condition.to_string())
            .set_expression_attribute_names(Some(attribute_names))
            .set_expression_attribute_values(Some(attribute_values))
            .send()
            .await
            .map(|_| ())
            .map_err(|e| {
                eprintln!(
                    "Error : {:?},  Key: {:?}, update expression: {:?}, condition expression: {:?}",
                    e, key_value, update, condition
                );
                Error::sdk_error("Dynamodb delete item failed.", e)
            })
    }

    pub async fn delete(&self, key_value: KeyValue) -> Result<(), Error> {
        self.client
            .delete_item()
            .table_name(self.table_name.clone())
            .set_key(Some(key_value.clone().into_hash_map()))
            .send()
            .await
            .map(|_| ())
            .map_err(|e| {
                eprintln!("Error : {:?},  Key: {:?} ", e, key_value);
                Error::sdk_error("Dynamodb delete item failed.", e)
            })
    }

    pub async fn delete_with_sort_key<T: AsPkAvailableCompositeKeyValue>(
        &self,
        sort_key_value: String,
    ) -> Result<(), Error> {
        let key_value = T::as_pk_available_composite_key_value(sort_key_value);
        self.delete(key_value).await
    }

    pub async fn delete_with_condition(
        &self,
        key_value: KeyValue,
        conditional_expression: ConditionalExpression,
    ) -> Result<(), Error> {
        self.client
            .delete_item()
            .table_name(self.table_name.clone())
            .set_key(Some(key_value.clone().into_hash_map()))
            .condition_expression(conditional_expression.to_string())
            .set_expression_attribute_names(Some(
                conditional_expression.get_expression_attribute_names(),
            ))
            .set_expression_attribute_values(Some(
                conditional_expression.get_expression_attribute_values(),
            ))
            .send()
            .await
            .map(|_| ())
            .map_err(|e| {
                eprintln!(
                    "Error : {:?},  Key: {:?} Conditional expression: {:?}",
                    e, key_value, conditional_expression
                );
                Error::sdk_error("Dynamodb delete item failed.", e)
            })
    }

    /// Performs batch_write in parallel. `parallel_count` dertermines how many parallel batch_wirte is called. <br>
    /// Calling sdk's batch_write in parallel comes with the risk of hitting throughput limit quicker. <br>
    /// Unproccessed failures behvaiour is same as `batch_write` function. <br>
    /// If any call fails, error is returned without continuing. <br>
    pub async fn parallel_batch_write(
        &self,
        items: Vec<Operation>,
        max_retry: usize,
        parallel_count: usize,
    ) -> Result<(), Error> {
        let mut final_unprocessed: HashMap<String, Vec<WriteRequest>> = HashMap::new();
        for parallel_batch_chunk in batch_chunks(batch_chunks(items, 25), parallel_count) {
            let mut tasks: Vec<_> = vec![];
            for x in parallel_batch_chunk {
                tasks.push(self._batch_write(
                    self.operations_into_write_requests(x)?,
                    max_retry,
                    None,
                ));
            }

            for result in join_all(tasks).await {
                if let Err(e) = result {
                    match &e {
                        Error::BatchOperationAbandon { unprocessed_items } => {
                            for unprocessed_item in unprocessed_items {
                                final_unprocessed
                                    .entry(unprocessed_item.0.clone())
                                    .or_default()
                                    .append(&mut unprocessed_item.1.clone());
                            }
                        }
                        _ => {
                            eprintln!("Error while performing parallel batch write : {}", e);
                            return Err(e);
                        }
                    }
                }
            }
        }

        if final_unprocessed.len() > 0 {
            return Err(Error::BatchOperationAbandon {
                unprocessed_items: final_unprocessed,
            });
        }

        return Ok(());
    }

    /// Aws sdk's batch write will only handle 25 records at a time. <br>
    /// This function will call the aws sdk's batch write back to back if more that 25 items have been provided.<br>
    /// Unprocessed data is treated as failure result as this function already handles retries.<br>
    /// Unprocessed failure will be returned only after handling all requests. Other failure will be returned immediately.<br>
    /// `parallel_batch_write` will call sdk batch write in parallel for all the record bundles, with the risk for hitting throughput limit quicker.<br>
    pub async fn batch_write(&self, items: Vec<Operation>, max_retry: usize) -> Result<(), Error> {
        let mut final_unprocessed: HashMap<String, Vec<WriteRequest>> = HashMap::new();
        for request_batch in batch_chunks(items, 25) {
            let result = self
                ._batch_write(
                    self.operations_into_write_requests(request_batch)?,
                    max_retry,
                    None,
                )
                .await;
            if let Err(e) = result {
                match &e {
                    Error::BatchOperationAbandon { unprocessed_items } => {
                        for unprocessed_item in unprocessed_items {
                            final_unprocessed
                                .entry(unprocessed_item.0.clone())
                                .or_default()
                                .append(&mut unprocessed_item.1.clone());
                        }
                    }
                    _ => {
                        return Err(e);
                    }
                }
            }
        }
        if final_unprocessed.len() > 0 {
            return Err(Error::BatchOperationAbandon {
                unprocessed_items: final_unprocessed,
            });
        }

        return Ok(());
    }

    fn operations_into_write_requests(
        &self,
        items: Vec<Operation>,
    ) -> Result<HashMap<String, Vec<WriteRequest>>, Error> {
        let mut requests: HashMap<String, Vec<WriteRequest>> = HashMap::new();
        for item in items {
            match item {
                Operation::Insert(data) => {
                    let put_request = WriteRequest::builder()
                        .put_request(
                            PutRequest::builder()
                                .set_item(Some(data.as_attribute_key_values()))
                                .build()
                                .map_err(|e| {
                                    Error::sdk_error("Error while building put request.", e)
                                })?,
                        )
                        .build();
                    requests
                        .entry(self.table_name.clone())
                        .or_default()
                        .push(put_request);
                }
                Operation::Delete(key_value) => {
                    let delete_request = WriteRequest::builder()
                        .delete_request(
                            DeleteRequest::builder()
                                .set_key(Some(key_value.into_hash_map()))
                                .build()
                                .map_err(|e| {
                                    Error::sdk_error("Error while building delete request.", e)
                                })?,
                        )
                        .build();
                    requests
                        .entry(self.table_name.clone())
                        .or_default()
                        .push(delete_request);
                }
            }
        }
        return Ok(requests);
    }

    #[async_recursion]
    async fn _batch_write(
        &self,
        items: HashMap<String, Vec<WriteRequest>>,
        max_retry: usize,
        wait_before: Option<Duration>,
    ) -> Result<(), Error> {
        if items.is_empty() {
            return Ok(());
        }

        if let Some(wait_before) = wait_before {
            sleep(wait_before).await;
        }

        let result = self
            .client
            .batch_write_item()
            .set_request_items(Some(items))
            .send()
            .await
            .map_err(|e| {
                eprintln!("Error : {:?}", e);
                Error::sdk_error("Dynamodb batch write failed.", e)
            })?;

        if let Some(unprocessed_items) = result.unprocessed_items {
            if max_retry != 0 {
                let wait_before = wait_before.map_or(Duration::from_millis(100), |x| {
                    let duration = x.saturating_mul(2);
                    let mut rng = rand::rng();
                    let jitter_ms = rng.random_range(0..=duration.as_millis() as u64);
                    Duration::from_millis(jitter_ms)
                });

                return self
                    ._batch_write(unprocessed_items, max_retry - 1, Some(wait_before))
                    .await;
            }
            return Err(Error::BatchOperationAbandon { unprocessed_items });
        }
        return Ok(());
    }
}

fn batch_chunks<T>(mut items: Vec<T>, batch_size: usize) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = vec![];
    let mut batch: Vec<T> = vec![];

    for item in items.drain(..) {
        batch.push(item);
        if batch.len() == batch_size {
            result.push(batch);
            batch = vec![];
        }
    }

    if !batch.is_empty() {
        result.push(batch);
    }

    result
}
