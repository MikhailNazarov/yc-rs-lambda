use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub messages: Vec<EventMessage>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventMessage {
    pub metadata: EventMetadata,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventMetadata {
    pub event_id: String,
    pub event_type: String,
    pub created_at: DateTime<Utc>,
    pub cloud_id: String,
    pub folder_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MessageDetails {
    UnsupportedMessage,
    QueueMessage(QueueMessageDetails),
}

const QUEUE_MESSAGE_EVENT: &str = "yandex.cloud.events.messagequeue.QueueMessage";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueueMessageDetails {
    pub queue_id: String,
    pub message: QueueMessage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueueMessage {
    pub message_id: String,
    pub md5_of_body: String,
    pub body: String,
    //pub attributes: HashMap<String, String>,
    //pub message_attributes: TODO,
    pub md5_of_message_attributes: String,
}
