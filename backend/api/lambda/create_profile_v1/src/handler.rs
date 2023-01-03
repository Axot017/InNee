use crate::dto::create_profile_dto::{CreateProfileDto, CreateProfileDtoWithId};
use common_api::{
    from_request::FromRequest, into_response::IntoResponse, user_context::UserContext,
};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use profile_domain::model::create_profile_params::CreateProfileParams;
use use_case::create_profile::{create_profile, CreateProfileRepository};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    CreateProfileDto::from_request(&event)
        .and_then(|dto| Ok(CreateProfileDtoWithId::new(event.get_user_id()?, dto)))
        .map(CreateProfileParams::from)
        .into_future()
        .and_then(|params| {
            create_profile(
                params,
                CreateProfileRepository {
                    get_profile_by_id: profile_infra::repository::get_profile_by_id,
                    save_profile: profile_infra::repository::save_profile,
                },
            )
        })
        .await
        .into_empty_response(StatusCode::CREATED)
}
