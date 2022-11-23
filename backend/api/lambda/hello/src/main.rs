use lambda_http::{run, tower::ServiceBuilder, Body, Error, Request, Response};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello World".into())
        .map_err(Box::new)?;
    Ok(resp)
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
