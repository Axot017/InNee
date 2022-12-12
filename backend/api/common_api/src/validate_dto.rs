use std::collections::HashMap;

use common_domain::error::{Error, ErrorOutput, ErrorType, Result};
use validator::Validate;

pub trait ValidateDto {
    fn validate_dto(&self) -> Result<()>;
}

impl<T> ValidateDto for T
where
    T: Validate,
{
    fn validate_dto(&self) -> Result<()> {
        self.validate().map_err(|e| {
            let args: HashMap<String, String> = e
                .errors()
                .iter()
                .flat_map(|(key, value)| {
                    serde_json::to_string(&value).map(|v| ((*key).to_owned(), v))
                })
                .collect();

            Error {
                debug_message: format!("Validation error: {e:?}"),
                error_type: ErrorType::InvalidInput,
                output: Box::new(ErrorOutput {
                    message: "Validation failure".to_owned(),
                    code: "validation_failure".to_owned(),
                    args,
                }),
            }
        })
    }
}
