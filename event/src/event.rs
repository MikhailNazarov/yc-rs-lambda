use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::vec::Vec;

use crate::details::MessageDetails;

#[derive(Debug, Clone, Deserialize)]
pub struct Event {
    pub messages: Vec<EventMessage>,
}

#[derive(Debug, Clone)]
pub struct EventMessage {
    pub metadata: EventMetadata,
    pub details: MessageDetails,
}

impl EventMessage {
    pub fn new(metadata: EventMetadata, details: MessageDetails) -> Self {
        EventMessage { metadata, details }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventMetadata {
    pub event_id: String,
    pub event_type: String,
    pub created_at: DateTime<Utc>,
    pub cloud_id: String,
    pub folder_id: String,
}
