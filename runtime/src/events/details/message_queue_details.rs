use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct QueueMessageDetails {
    pub queue_id: String,
    pub message: QueueMessage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueueMessage {
    pub message_id: String,
    pub md5_of_body: String,
    pub body: String,
    //pub attributes: HashMap<String, String>,
    //pub message_attributes: TODO,
    pub md5_of_message_attributes: String,
}
