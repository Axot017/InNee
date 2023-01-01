use common_api::{into_response::IntoResponse, user_context::UserContext};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use profile_infra::repository;
use use_case::get_profile_by_id::{get_profile_by_id, GetProfileByIdRepository};

use crate::dto::profile_dto::ProfileDto;

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    event
        .get_user_id()
        .into_future()
        .and_then(|id| {
            get_profile_by_id(
                id,
                GetProfileByIdRepository {
                    get_profile_by_id: repository::get_profile_by_id,
                },
            )
        })
        .await
        .into_response::<ProfileDto>(StatusCode::OK)
}
