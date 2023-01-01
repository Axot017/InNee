use crate::error_output_dto::ErrorOutputDto;
use common_domain::error::{Error, ErrorType};
use http::StatusCode;
use lambda_http::{Body, Response};

#[derive(PartialEq, Eq, Debug)]
pub struct ErrorDto {
    pub status_code: StatusCode,
    pub error_dto: ErrorOutputDto,
}

impl From<Error> for ErrorDto {
    fn from(error: Error) -> Self {
        Self {
            status_code: match error.error_type {
                ErrorType::InvalidInput => StatusCode::BAD_REQUEST,
                ErrorType::Conflict => StatusCode::CONFLICT,
                ErrorType::NotFound => StatusCode::NOT_FOUND,
                ErrorType::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            },
            error_dto: (*error.output).into(),
        }
    }
}

impl TryFrom<ErrorDto> for Response<Body> {
    type Error = lambda_http::Error;

    fn try_from(error: ErrorDto) -> Result<Self, Self::Error> {
        let result = Response::builder()
            .status(error.status_code)
            .header(
                http::header::CONTENT_TYPE,
                mime::APPLICATION_JSON.to_string(),
            )
            .body(
                serde_json::to_string(&error.error_dto)
                    .map_err(Box::new)?
                    .into(),
            )
            .map_err(Box::new)?;

        Ok(result)
    }
}
