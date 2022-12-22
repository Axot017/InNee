use log::Level;

use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Error>;

const UNKNOWN_ERROR_MESSAGE: &str = "Unknown server error";
const UNKNOWN_ERROR_CODE: &str = "unknown";

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ErrorType {
    InvalidInput,
    Conflict,
    Unknown,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error {
    pub debug_message: String,
    pub error_type: ErrorType,
    pub output: Box<ErrorOutput>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ErrorOutput {
    pub message: String,
    pub code: String,
    pub args: HashMap<String, String>,
}

impl Default for ErrorType {
    fn default() -> Self {
        ErrorType::Unknown
    }
}

impl Default for Error {
    fn default() -> Self {
        Self {
            debug_message: "".to_owned(),
            error_type: ErrorType::Unknown,
            output: Default::default(),
        }
    }
}

impl Default for ErrorOutput {
    fn default() -> Self {
        Self {
            message: UNKNOWN_ERROR_MESSAGE.to_owned(),
            code: UNKNOWN_ERROR_CODE.to_owned(),
            args: HashMap::new(),
        }
    }
}

impl Error {
    pub fn unknown(message: String) -> Self {
        Self {
            debug_message: message,
            ..Default::default()
        }
    }
}

impl From<ErrorType> for Level {
    fn from(ty: ErrorType) -> Self {
        match ty {
            ErrorType::InvalidInput => Level::Info,
            ErrorType::Unknown => Level::Error,
            ErrorType::Conflict => Level::Info,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn default_value() {
        let value = Error::default();

        assert_eq!(
            value,
            Error {
                debug_message: "".to_owned(),
                error_type: ErrorType::Unknown,
                output: Box::new(ErrorOutput {
                    message: UNKNOWN_ERROR_MESSAGE.to_owned(),
                    code: UNKNOWN_ERROR_CODE.to_owned(),
                    args: HashMap::new(),
                })
            }
        )
    }

    #[test]
    fn unknown() {
        let value = Error::unknown("Custom message".to_owned());

        assert_eq!(
            value,
            Error {
                debug_message: "Custom message".to_owned(),
                error_type: ErrorType::Unknown,
                output: Box::new(ErrorOutput {
                    message: UNKNOWN_ERROR_MESSAGE.to_owned(),
                    code: UNKNOWN_ERROR_CODE.to_owned(),
                    args: HashMap::new(),
                })
            }
        )
    }

    #[test]
    fn default_output() {
        let out = ErrorOutput::default();

        assert_eq!(
            out,
            ErrorOutput {
                message: UNKNOWN_ERROR_MESSAGE.to_owned(),
                code: UNKNOWN_ERROR_CODE.to_owned(),
                args: HashMap::new(),
            }
        )
    }
}
