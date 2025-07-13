use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushNotification {
    pub user_id: String,
    pub title: String,
    pub body: Value,      
    pub timestamp: u64,
}
