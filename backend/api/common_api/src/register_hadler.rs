use std::future::Future;

use http::Response;
use lambda_http::{run, tower::ServiceBuilder, Body, Error, Request};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub async fn register_handler<'a, F, FUT>(handler: F) -> Result<(), Error>
where
    FUT: Future<Output = Result<Response<Body>, Error>> + Send,
    F: FnMut(Request) -> FUT + Send + 'a,
{
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .without_time()
        .init();

    let layer = TraceLayer::new_for_http()
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let service = ServiceBuilder::new().layer(layer).service_fn(handler);

    run(service).await
}
