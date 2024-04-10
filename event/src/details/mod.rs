use serde::Deserialize;

use self::{message_queue_details::QueueMessageDetails, timer_details::TimerDetails};

pub mod message_queue_details;
pub mod timer_details;

#[derive(Debug, Clone, Deserialize)]
pub enum MessageDetails {
    UnsupportedMessage,
    QueueMessage(QueueMessageDetails),
    TimerMessage(TimerDetails),
}
