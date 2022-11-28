use common_domain::error::ErrorOutput;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct ErrorOutputDto {
    pub message: String,
    pub code: String,
    pub args: HashMap<String, String>,
}

impl From<ErrorOutput> for ErrorOutputDto {
    fn from(error: ErrorOutput) -> Self {
        Self {
            message: error.message,
            code: error.code,
            args: error.args,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_error_details() {
        let message = "Test message".to_owned();
        let code = "error.test_code".to_owned();
        let args = HashMap::from([("key".to_owned(), "value".to_owned())]);
        let error_details = ErrorOutput {
            message: message.clone(),
            code: code.clone(),
            args: args.clone(),
        };

        assert_eq!(
            ErrorOutputDto::from(error_details),
            ErrorOutputDto {
                message,
                code,
                args
            }
        );
    }
}
