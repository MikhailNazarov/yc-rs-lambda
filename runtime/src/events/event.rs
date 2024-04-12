use actix_web::{web::Json, FromRequest};
use chrono::{DateTime, Utc};
use futures::Future;
use serde::Deserialize;
use std::{pin::Pin, vec::Vec};

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

impl FromRequest for Event {
    type Error = actix_web::error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();
        let mut p = payload.take();
        Box::pin(async move { Json::<Event>::from_request(&req, &mut p).await.map(|x| x.0) })
    }
}
