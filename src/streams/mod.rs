pub mod from_image;
mod handle_event_records;
use aws_lambda_events::dynamodb::{EventRecord, StreamRecord};
pub use handle_event_records::HandleEventRecords;

use serde_dynamo::Item;

use crate::{key::KeyValue, streams::from_image::FromImage};

pub enum EventName {
    Insert { new_image: Item },
    Remove { old_image: Item },
    Modify { new_image: Item, old_image: Item },
}

impl TryFrom<&EventRecord> for EventName {
    type Error = String;
    fn try_from(value: &EventRecord) -> Result<Self, Self::Error> {
        let EventRecord {
            change:
                StreamRecord {
                    new_image,
                    old_image,
                    ..
                },
            event_name,
            ..
        } = value.clone();

        match event_name.as_str() {
            "INSERT" => Ok(EventName::Insert { new_image }),
            "REMOVE" => Ok(EventName::Remove { old_image }),
            "MODIFY" => Ok(EventName::Modify {
                new_image,
                old_image,
            }),
            _ => {
                return Err("Unhandled event name.".to_string());
            }
        }
    }
}

pub struct ModifiedItem<T> {
    pub old: T,
    pub new: T,
}

impl<T> ModifiedItem<T> {
    pub fn new(old: T, new: T) -> Self {
        Self { old, new }
    }
}

#[async_trait::async_trait]
pub trait EventHandler<T>: Send + Sync
where
    T: FromImage<T>,
{
    fn key_value_belongs(key_value: &KeyValue) -> bool;
    fn ignore_modify(item: &ModifiedItem<T>) -> bool;
    async fn handle_insert(items: Vec<T>);
    async fn handle_remove(items: Vec<T>);
    async fn handle_modify(items: Vec<ModifiedItem<T>>);
}
