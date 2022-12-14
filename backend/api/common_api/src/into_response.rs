use http::StatusCode;
use lambda_http::{Body, Error, Response};
use serde::Serialize;

use crate::error_dto::ErrorDto;

pub trait IntoResponse<T> {
    fn into_response<S>(self, status_code: StatusCode) -> Result<Response<Body>, Error>
    where
        S: From<T> + Serialize;

    fn into_empty_response(self, status_code: StatusCode) -> Result<Response<Body>, Error>;
}

impl<T> IntoResponse<T> for Result<T, common_domain::error::Error> {
    fn into_response<S>(self, status_code: StatusCode) -> Result<Response<Body>, Error>
    where
        S: From<T> + serde::Serialize,
    {
        match self {
            Ok(value) => Ok(Response::builder()
                .status(status_code)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&S::from(value))
                        .map_err(Box::new)?
                        .into(),
                )
                .map_err(Box::new)?),
            Err(err) => ErrorDto::from(err).try_into(),
        }
    }

    fn into_empty_response(self, status_code: StatusCode) -> Result<Response<Body>, Error> {
        match self {
            Ok(_) => Ok(Response::builder()
                .status(status_code)
                .body(Body::Empty)
                .map_err(Box::new)?),
            Err(err) => ErrorDto::from(err).try_into(),
        }
    }
}
