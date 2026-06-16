use aws_lambda_events::dynamodb::EventRecord;

use crate::{
    key::KeyValue,
    streams::{EventHandler, EventName, ModifiedItem, from_image::FromImage},
    traits::has_key::HasKey,
};

#[async_trait::async_trait]
pub trait HandleEventRecords {
    async fn handle<TEvent: FromImage<TEvent> + HasKey, THandler: EventHandler<TEvent>>(
        mut self,
    ) -> Self;
}

#[async_trait::async_trait]
impl HandleEventRecords for Vec<EventRecord> {
    async fn handle<TEvent: FromImage<TEvent> + HasKey, THandler: EventHandler<TEvent>>(
        mut self,
    ) -> Self {
        let mut records = vec![];
        self.retain(|x| {
            let Ok(key_value) = KeyValue::from_event_record(x.clone(), TEvent::get_key()) else {
                return false;
            };
            if THandler::key_value_belongs(&key_value) {
                records.push(x.clone());
                return false;
            }
            return true;
        });

        let events: Vec<EventName> = records
            .iter()
            .filter_map(|x| match EventName::try_from(x) {
                Ok(x) => Some(x),
                Err(_) => {
                    eprintln!("Error while extracting event name for : {:?}", x);
                    None
                }
            })
            .collect();

        let inserts: Vec<TEvent> = events
            .iter()
            .filter_map(|x| {
                if let EventName::Insert { new_image } = x {
                    TEvent::from_image(new_image.clone()).ok()
                } else {
                    None
                }
            })
            .collect();
        THandler::handle_insert(inserts).await;

        let removes: Vec<TEvent> = events
            .iter()
            .filter_map(|x| {
                if let EventName::Remove { old_image } = x {
                    TEvent::from_image(old_image.clone()).ok()
                } else {
                    None
                }
            })
            .collect();
        THandler::handle_remove(removes).await;

        let modified: Vec<ModifiedItem<TEvent>> = events
            .iter()
            .filter_map(|x| {
                if let EventName::Modify {
                    new_image,
                    old_image,
                } = x
                {
                    let Ok(old) = TEvent::from_image(old_image.clone()) else {
                        return None;
                    };

                    let Ok(new) = TEvent::from_image(new_image.clone()) else {
                        return None;
                    };

                    return Some(ModifiedItem::new(old, new));
                } else {
                    return None;
                }
            })
            .collect();
        THandler::handle_modify(modified).await;

        self
    }
}
