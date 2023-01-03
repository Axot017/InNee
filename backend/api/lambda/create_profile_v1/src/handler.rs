use lambda_http::{Body, Error, Request, Response};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    let _a = event;
    todo!()
}
