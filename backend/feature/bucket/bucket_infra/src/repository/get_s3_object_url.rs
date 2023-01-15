use common_domain::error::{Error, Result};
use common_infra::config::CONFIG;

pub async fn get_s3_object_url(key: &str) -> Result<String> {
    aws_config::load_from_env()
        .await
        .region()
        .ok_or_else(|| Error::unknown("Region not found".to_owned()))
        .map(|region| {
            format!(
                "https://{}.s3.{}.amazonaws.com/{}",
                &CONFIG.s3_bucket, region, key
            )
        })
}
