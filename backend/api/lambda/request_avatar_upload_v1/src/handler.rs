use common_api::{
    dto::presigned_url_dto::PresignedUrlDto, into_response::IntoResponse, user_context::UserContext,
};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::request_avatar_upload::{request_avatar_upload, RequestAvatarUploadRepository};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    event
        .get_user_id()
        .into_future()
        .and_then(|id| {
            request_avatar_upload(
                id,
                RequestAvatarUploadRepository {
                    get_profile_by_id: profile_infra::repository::get_profile_by_id,
                    get_avatar_upload_url: profile_infra::repository::get_avatar_upload_url,
                },
            )
        })
        .await
        .into_response::<PresignedUrlDto>(StatusCode::OK)
}
