pub mod event_message;

use serde::de::MapAccess;

use crate::{details::MessageDetails, EventMetadata};

const QUEUE_MESSAGE_EVENT: &str = "yandex.cloud.events.messagequeue.QueueMessage";

fn deserialize_details<'de, V>(
    metadata: &EventMetadata,
    mut map: &mut V,
) -> Result<MessageDetails, V::Error>
where
    V: MapAccess<'de>,
{
    let value = match metadata.event_type.as_str() {
        QUEUE_MESSAGE_EVENT => MessageDetails::QueueMessage(map.next_value()?),
        _ => MessageDetails::UnsupportedMessage,
    };
    Ok(value)
}
