use std::collections::HashMap;

use common_domain::models::PresignedUrl;
use lambda_http::aws_lambda_events::chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Clone, Debug)]
pub struct PresignedUrlDto {
    url: String,
    valid_until: DateTime<Utc>,
    headers: HashMap<String, String>,
}

impl From<PresignedUrl> for PresignedUrlDto {
    fn from(presigned_url: PresignedUrl) -> Self {
        Self {
            url: presigned_url.url,
            valid_until: presigned_url.valid_until,
            headers: presigned_url.headers,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_from_presigned_url() {
        let presigned_url = PresignedUrl {
            url: "https://example.com".to_string(),
            valid_until: Utc::now(),
            headers: HashMap::from([("key".to_string(), "value".to_string())]),
        };
        let presigned_url_dto = PresignedUrlDto::from(presigned_url.clone());
        assert_eq!(presigned_url_dto.url, presigned_url.url);
        assert_eq!(presigned_url_dto.valid_until, presigned_url.valid_until);
        assert_eq!(presigned_url_dto.headers, presigned_url.headers);
    }
}
