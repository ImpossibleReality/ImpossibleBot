use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
/// Guild In Database
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Guild {
    /// Discord Guild ID
    id: String,
}

/// Member In Database
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Member {
    /// Discord guild ID
    guild_id: String,
    /// Discord user ID
    user_id: String,
    /// Number of points user has
    #[serde(default)]
    points: u8,
}

/// Counting Channel In Database
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Channel {
    pub id: String,
    pub current_number: u64,
    pub last_count_user: Option<String>,
    pub last_count_time: Option<i64>,
    pub last_count_message_id: Option<String>,
}
