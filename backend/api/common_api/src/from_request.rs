use common_domain::error::{Error, ErrorOutput, ErrorType};
use lambda_http::Request;
use serde::de::DeserializeOwned;

pub trait FromRequest {
    fn from_request(request: &Request) -> Result<Self, Error>
    where
        Self: Sized;
}

impl<T> FromRequest for T
where
    T: DeserializeOwned,
{
    fn from_request(request: &Request) -> Result<Self, Error> {
        serde_json::from_slice::<T>(request.body()).map_err(|e| Error {
            debug_message: e.to_string(),
            error_type: ErrorType::InvalidInput,
            output: Box::new(ErrorOutput {
                message: "Invalid request format".to_owned(),
                code: "invalid_request_format".to_owned(),
                ..Default::default()
            }),
        })
    }
}
