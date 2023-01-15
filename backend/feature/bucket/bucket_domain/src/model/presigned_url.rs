use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PresignedUrl {
    pub url: String,
    pub valid_until: DateTime<Utc>,
    pub headers: HashMap<String, String>,
}
