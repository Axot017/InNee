use bucket_domain::{model::presigned_url::PresignedUrl, port::GetUploadUrl};
use common_domain::error::{Error, ErrorOutput, ErrorType, Result};
use profile_domain::{consts::AVATAR_IMAGE_PREFIX, port::GetProfileById};

pub struct RequestAvatarUploadRepository<A, B> {
    pub get_profile_by_id: A,
    pub get_avatar_upload_url: B,
}

pub async fn request_avatar_upload<A, B>(
    profile_id: String,
    repo: RequestAvatarUploadRepository<A, B>,
) -> Result<PresignedUrl>
where
    for<'a> A: GetProfileById<'a>,
    for<'a> B: GetUploadUrl<'a>,
{
    let profile = (repo.get_profile_by_id)(&profile_id).await?;

    if profile.is_none() {
        return Err(profile_not_created(&profile_id));
    }

    (repo.get_avatar_upload_url)(&format!("{}{}", AVATAR_IMAGE_PREFIX, profile_id)).await
}

fn profile_not_created(profile_id: &str) -> Error {
    Error {
        debug_message: format!("Profile {} not created, cannot upload avatar", profile_id),
        error_type: ErrorType::Conflict,
        output: Box::new(ErrorOutput {
            message: "Profile not created, cannot upload avatar".to_owned(),
            code: "profile_not_created".to_owned(),
            ..Default::default()
        }),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use common_domain::tokio;
    use profile_domain::model::profile::Profile;

    use super::*;

    #[tokio::test]
    async fn should_return_error_when_profile_not_created() {
        let id = "id";
        let _get_profile_id_lock = profile_domain::port::get_profile_by_id_lock().await;
        let ctx = profile_domain::port::mock_get_profile_by_id::call_context();
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(|_| Ok(None));

        let _get_avatar_upload_url_lock = bucket_domain::port::get_upload_url_lock().await;
        let ctx = bucket_domain::port::mock_get_upload_url::call_context();
        ctx.expect().times(0);

        let result = request_avatar_upload(
            id.to_owned(),
            RequestAvatarUploadRepository {
                get_profile_by_id: profile_domain::port::mock_get_profile_by_id::call,
                get_avatar_upload_url: bucket_domain::port::mock_get_upload_url::call,
            },
        )
        .await;

        assert!(result.is_err());
        assert_eq!(result.as_ref().unwrap_err().error_type, ErrorType::Conflict);
        assert_eq!(result.unwrap_err(), profile_not_created(id));
    }

    #[tokio::test]
    async fn should_return_presigned_url() {
        let date_time = chrono::Utc::now();
        let id = "id";
        let _get_profile_id_lock = profile_domain::port::get_profile_by_id_lock().await;
        let ctx = profile_domain::port::mock_get_profile_by_id::call_context();
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(|_| Ok(Some(Profile::default())));

        let _get_avatar_upload_url_lock = bucket_domain::port::get_upload_url_lock().await;
        let ctx = bucket_domain::port::mock_get_upload_url::call_context();
        ctx.expect()
            .withf(move |id| id == "profile/avatar/id")
            .times(1)
            .returning(move |_| {
                Ok(PresignedUrl {
                    url: "url".to_owned(),
                    valid_until: date_time,
                    headers: HashMap::new(),
                })
            });

        let result = request_avatar_upload(
            id.to_owned(),
            RequestAvatarUploadRepository {
                get_profile_by_id: profile_domain::port::mock_get_profile_by_id::call,
                get_avatar_upload_url: bucket_domain::port::mock_get_upload_url::call,
            },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            PresignedUrl {
                url: "url".to_owned(),
                valid_until: date_time,
                headers: HashMap::new(),
            }
        );
    }
}
