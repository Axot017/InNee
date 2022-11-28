use common_domain::error::ErrorOutput;
use lambda_http::{run, tower::ServiceBuilder, Body, Error, Request, Response};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let layer = TraceLayer::new_for_http()
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let service = ServiceBuilder::new()
        .layer(layer)
        .service_fn(function_handler);

    run(service).await
}
