use std::fmt;

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize,
};

use crate::{events::deserialize::deserialize_details, EventMessage, EventMetadata};

impl<'de> Deserialize<'de> for EventMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Metadata,
            Details,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "event_metadata" => Ok(Field::Metadata),
                            "details" => Ok(Field::Details),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct EventMessageVisitor;

        impl<'de> Visitor<'de> for EventMessageVisitor {
            type Value = EventMessage;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct EventMessage")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<EventMessage, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let secs = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let nanos = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(EventMessage::new(secs, nanos))
            }

            fn visit_map<V>(self, mut map: V) -> Result<EventMessage, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut metadata: Option<EventMetadata> = None;
                let mut details = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Metadata => {
                            if metadata.is_some() {
                                return Err(de::Error::duplicate_field("event_metadata"));
                            }
                            metadata = Some(map.next_value()?);
                        }
                        Field::Details => {
                            if details.is_some() {
                                return Err(de::Error::duplicate_field("details"));
                            }
                            if let Some(metadata) = &metadata {
                                let value = deserialize_details(metadata, &mut map)?;

                                details = Some(value);
                            } else {
                                return Err(de::Error::custom(
                                    "message details must be after message metadata",
                                ));
                            }
                        }
                    }
                }
                let metadata =
                    metadata.ok_or_else(|| de::Error::missing_field("event_metadata"))?;
                let details = details.ok_or_else(|| de::Error::missing_field("details"))?;
                Ok(EventMessage::new(metadata, details))
            }
        }

        const FIELDS: &[&str] = &["event_metadata", "details"];
        deserializer.deserialize_struct("EventMessage", FIELDS, EventMessageVisitor)
    }
}
