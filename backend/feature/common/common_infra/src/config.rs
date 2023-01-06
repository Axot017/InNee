use std::env;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}

pub struct Config {
    pub dynamodb_table: String,
    pub s3_bucket: String,
    pub presigned_url_duration_in_sec: u64,
}

impl Config {
    pub fn new() -> Self {
        let dynamodb_table =
            env::var("DYNAMODB_TABLE").unwrap_or_else(|_| "in-nee-dev".to_string());
        let s3_bucket = env::var("S3_BUCKET").unwrap_or_else(|_| "in-nee-dev".to_string());
        let presigned_url_duration_in_sec = env::var("PRESIGNED_URL_DURATION_IN_SEC")
            .map_err(|_| ())
            .and_then(|s| s.parse::<u64>().map_err(|_| ()))
            .unwrap_or(60);

        Self {
            dynamodb_table,
            s3_bucket,
            presigned_url_duration_in_sec,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {
        with_set_values();
        with_no_values();
    }

    fn with_set_values() {
        env::set_var("DYNAMODB_TABLE", "in-nee-test");
        env::set_var("S3_BUCKET", "in-nee-test");
        env::set_var("PRESIGNED_URL_DURATION_IN_SEC", "20");

        let config = Config::new();

        assert_eq!(&config.dynamodb_table, "in-nee-test");
        assert_eq!(&config.s3_bucket, "in-nee-test");
        assert_eq!(config.presigned_url_duration_in_sec, 20);
    }

    fn with_no_values() {
        env::remove_var("DYNAMODB_TABLE");
        env::remove_var("S3_BUCKET");
        env::remove_var("PRESIGNED_URL_DURATION_IN_SEC");

        let config = Config::new();

        assert_eq!(&config.dynamodb_table, "in-nee-dev");
        assert_eq!(&config.s3_bucket, "in-nee-dev");
        assert_eq!(config.presigned_url_duration_in_sec, 60);
    }
}
