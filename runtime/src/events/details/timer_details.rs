use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TimerDetails {
    pub trigger_id: String,
    pub payload: String,
}
