use lambda_http::{Body, Error, Request, Response};

pub async fn handle_request(_event: Request) -> Result<Response<Body>, Error> {
    todo!()
}
