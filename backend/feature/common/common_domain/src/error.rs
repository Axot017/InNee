use log::Level;

use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Error>;

const UNKNOWN_ERROR_MESSAGE: &str = "Unknown server error";
const UNKNOWN_ERROR_CODE: &str = "unknown";

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ErrorType {
    InvalidInput,
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

#[derive(Debug)]
pub struct ErrorOutputBuilder {
    message: Option<String>,
    code: Option<String>,
    args: HashMap<String, String>,
}

#[derive(Debug)]
pub struct ErrorBuilder {
    debug_message: Option<String>,
    error_type: Option<ErrorType>,
    output: Option<Box<ErrorOutput>>,
}

impl Default for ErrorType {
    fn default() -> Self {
        ErrorType::Unknown
    }
}

impl Default for Error {
    fn default() -> Self {
        ErrorBuilder::new().build()
    }
}

impl Default for ErrorOutput {
    fn default() -> Self {
        ErrorOutputBuilder::new().build()
    }
}

impl Default for ErrorOutputBuilder {
    fn default() -> Self {
        ErrorOutputBuilder::new()
    }
}

impl Default for ErrorBuilder {
    fn default() -> Self {
        ErrorBuilder::new()
    }
}

impl ErrorOutput {
    pub fn builder() -> ErrorOutputBuilder {
        ErrorOutputBuilder::new()
    }
}

impl ErrorOutputBuilder {
    pub fn new() -> Self {
        ErrorOutputBuilder {
            message: None,
            code: None,
            args: HashMap::new(),
        }
    }

    pub fn set_message(mut self, message: String) -> Self {
        self.message = Some(message);

        self
    }

    pub fn set_code(mut self, code: String) -> Self {
        self.code = Some(code);

        self
    }

    pub fn add_arg(mut self, key: String, value: String) -> Self {
        self.args.insert(key, value);

        self
    }

    pub fn build(self) -> ErrorOutput {
        ErrorOutput {
            message: self
                .message
                .unwrap_or_else(|| UNKNOWN_ERROR_MESSAGE.to_owned()),
            code: self.code.unwrap_or_else(|| UNKNOWN_ERROR_CODE.to_owned()),
            args: self.args,
        }
    }
}

impl Error {
    pub fn unknown(message: String) -> Error {
        Error::builder().set_debug_message(message).build()
    }

    pub fn builder() -> ErrorBuilder {
        ErrorBuilder::new()
    }
}

impl From<ErrorType> for Level {
    fn from(ty: ErrorType) -> Self {
        match ty {
            ErrorType::InvalidInput => Level::Info,
            ErrorType::Unknown => Level::Error,
        }
    }
}

impl ErrorBuilder {
    fn new() -> Self {
        Self {
            debug_message: None,
            error_type: None,
            output: None,
        }
    }

    pub fn set_debug_message(mut self, message: String) -> Self {
        self.debug_message = Some(message);

        self
    }

    pub fn set_error_type(mut self, error_type: ErrorType) -> Self {
        self.error_type = Some(error_type);

        self
    }

    pub fn set_output(mut self, details: ErrorOutput) -> Self {
        self.output = Some(Box::new(details));

        self
    }

    pub fn build(self) -> Error {
        let debug_message = self.debug_message.unwrap_or_default();
        let error_type = self.error_type.unwrap_or_default();
        let details = self.output.unwrap_or_default();

        log::log!(Level::from(error_type), "{debug_message:?} - {details:?}");

        Error {
            debug_message,
            error_type,
            output: details,
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
    fn builder() {
        let debug_message = "Debug message".to_owned();
        let error_type = ErrorType::InvalidInput;
        let output = ErrorOutput {
            message: "Error details message".to_owned(),
            code: "error.test_code".to_owned(),
            args: HashMap::from([("test".to_owned(), "arg".to_owned())]),
        };
        let value = Error::builder()
            .set_debug_message(debug_message.to_owned())
            .set_error_type(error_type)
            .set_output(output.clone())
            .build();

        assert_eq!(
            value,
            Error {
                debug_message,
                error_type,
                output: Box::new(output),
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

    #[test]
    fn output_builder() {
        let out = ErrorOutput::builder()
            .set_message("Test message".to_owned())
            .set_code("test".to_owned())
            .add_arg("test_key_1".to_owned(), "test_value_1".to_owned())
            .add_arg("test_key_2".to_owned(), "test_value_2".to_owned())
            .build();

        assert_eq!(
            out,
            ErrorOutput {
                message: "Test message".to_owned(),
                code: "test".to_owned(),
                args: HashMap::from([
                    ("test_key_1".to_owned(), "test_value_1".to_owned()),
                    ("test_key_2".to_owned(), "test_value_2".to_owned())
                ])
            }
        )
    }
}
