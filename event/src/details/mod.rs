use serde::Deserialize;

use self::message_queue_details::QueueMessageDetails;

pub mod message_queue_details;

#[derive(Debug, Clone, Deserialize)]
pub enum MessageDetails {
    UnsupportedMessage,
    QueueMessage(QueueMessageDetails),
}
