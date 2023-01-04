use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PresignedUrl {
    pub url: String,
    pub valid_until: DateTime<Utc>,
    pub headers: HashMap<String, String>,
}
