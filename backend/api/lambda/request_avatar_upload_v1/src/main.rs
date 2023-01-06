mod handler;

use common_api::register_hadler::register_handler;
use lambda_http::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handler::handle_request).await
}
