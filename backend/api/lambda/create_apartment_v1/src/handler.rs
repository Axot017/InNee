use common_domain::error::ErrorOutput;
use lambda_http::{Body, Error, Request, Response};

pub async fn handle_request(_event: Request) -> Result<Response<Body>, Error> {
    common_api::error_dto::ErrorDto::from(
        common_domain::error::Error::builder()
            .set_debug_message("test".to_owned())
            .set_error_type(common_domain::error::ErrorType::InvalidInput)
            .set_output(
                ErrorOutput::builder()
                    .set_code("test".to_owned())
                    .set_message("Test message".to_owned())
                    .add_arg("Test arg".to_owned(), "Test value".to_owned())
                    .build(),
            )
            .build(),
    )
    .try_into()
}
