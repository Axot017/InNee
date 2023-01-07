mod handler;

use common_api::register_internal_handler::register_internal_handler;
use handler::handle_event;
use lambda_runtime::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_internal_handler(handle_event).await
}
