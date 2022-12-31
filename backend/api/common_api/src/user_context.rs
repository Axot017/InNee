use common_domain::error::{Error, Result};
use lambda_http::{request::RequestContext, Request, RequestExt};

const USER_ID_KEY: &str = "sub";

pub trait UserContext {
    fn get_user_id(&self) -> Result<String>;
}

#[cfg(feature = "lambda")]
impl UserContext for Request {
    fn get_user_id(&self) -> Result<String> {
        match self.request_context() {
            RequestContext::ApiGatewayV1(context) => context
                .authorizer
                .get(USER_ID_KEY)
                .and_then(|sub| sub.as_str())
                .map(|sub| sub.replace('-', ""))
                .ok_or_else(|| Error::unknown("User ID not found in request context".to_owned())),
        }
    }
}
