use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use std::future::Future;

pub async fn register_internal_handler<'a, F, FUT, T>(handler: F) -> Result<(), Error>
where
    FUT: Future<Output = Result<(), Error>> + Send,
    F: FnMut(LambdaEvent<T>) -> FUT + Send + 'a,
    for<'b> T: serde::Deserialize<'b>,
{
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(handler)).await
}
