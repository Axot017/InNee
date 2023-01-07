use bucket_domain::port::{DeleteBucketObject, GetBucketObjectInfo, GetBucketObjectUrl};
use common_domain::error::{Error, Result};
use profile_domain::port::UpdateProfileAvatar;

const ALLOWED_CONTENT_TYPES: [&str; 2] = ["image/png", "image/jpeg"];
const MAX_SIZE: u64 = 1024 * 1024 * 3; // 3 MB

pub struct OnAvatarCreatedRepository<A, B, C, D> {
    pub get_bucket_object_info: A,
    pub delete_bucket_object: B,
    pub update_profile_avatar: C,
    pub get_bucket_object_url: D,
}

pub async fn on_avatar_created<A, B, C, D>(
    key: String,
    repo: OnAvatarCreatedRepository<A, B, C, D>,
) -> Result<()>
where
    for<'a> A: GetBucketObjectInfo<'a>,
    for<'a> B: DeleteBucketObject<'a>,
    for<'a> C: UpdateProfileAvatar<'a>,
    for<'a> D: GetBucketObjectUrl<'a>,
{
    let bucket_object_head = (repo.get_bucket_object_info)(&key).await?;
    let id = key
        .split('/')
        .last()
        .ok_or_else(|| Error::unknown("should never happend".to_owned()))?;

    if !ALLOWED_CONTENT_TYPES.contains(&bucket_object_head.mime.as_str())
        || bucket_object_head.size > MAX_SIZE
    {
        log::info!("Deleting invalid avatar {}: {:?}", key, bucket_object_head);
        (repo.update_profile_avatar)(id, None).await?;

        return (repo.delete_bucket_object)(&key).await;
    }
    let avatar_url = (repo.get_bucket_object_url)(&key).await?;

    (repo.update_profile_avatar)(id, Some(&avatar_url)).await
}

#[cfg(test)]
mod test {
    use bucket_domain::model::bucket_object_head::BucketObjectHead;
    use common_domain::tokio;

    use super::*;

    #[tokio::test]
    async fn avatar_mime_not_allowed() {
        let key = "profile/avatar/1".to_string();

        let _ = bucket_domain::port::get_bucket_object_info_lock();
        let ctx = bucket_domain::port::mock_get_bucket_object_info::call_context();
        let object_head = BucketObjectHead {
            mime: "image/gif".to_string(),
            ..Default::default()
        };
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .return_once(move |_| Ok(object_head));

        let _ = profile_domain::port::update_profile_avatar_lock();
        let ctx = profile_domain::port::mock_update_profile_avatar::call_context();
        ctx.expect()
            .withf(move |id, avatar_url| id == "1" && avatar_url.is_none())
            .times(1)
            .return_once(move |_, _| Ok(()));

        let _ = bucket_domain::port::delete_bucket_object_lock();
        let ctx = bucket_domain::port::mock_delete_bucket_object::call_context();
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .returning(|_| Ok(()));

        let _ = profile_domain::port::update_profile_avatar_lock();
        let ctx = profile_domain::port::mock_update_profile_avatar::call_context();
        ctx.expect().times(0);

        let _ = bucket_domain::port::get_bucket_object_url_lock();
        let ctx = bucket_domain::port::mock_get_bucket_object_url::call_context();
        ctx.expect().times(0);

        let repo = OnAvatarCreatedRepository {
            get_bucket_object_info: bucket_domain::port::mock_get_bucket_object_info::call,
            delete_bucket_object: bucket_domain::port::mock_delete_bucket_object::call,
            update_profile_avatar: profile_domain::port::mock_update_profile_avatar::call,
            get_bucket_object_url: bucket_domain::port::mock_get_bucket_object_url::call,
        };

        let result = on_avatar_created(key, repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn avatar_too_big() {
        let key = "profile/avatar/1".to_string();

        let _ = bucket_domain::port::get_bucket_object_info_lock();
        let ctx = bucket_domain::port::mock_get_bucket_object_info::call_context();
        let object_head = BucketObjectHead {
            mime: "image/png".to_string(),
            size: 1024 * 1024 * 4,
            ..Default::default()
        };
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .return_once(move |_| Ok(object_head));

        let _ = profile_domain::port::update_profile_avatar_lock();
        let ctx = profile_domain::port::mock_update_profile_avatar::call_context();
        ctx.expect()
            .withf(move |id, avatar_url| id == "1" && avatar_url.is_none())
            .times(1)
            .return_once(move |_, _| Ok(()));

        let _ = bucket_domain::port::delete_bucket_object_lock();
        let ctx = bucket_domain::port::mock_delete_bucket_object::call_context();
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .returning(|_| Ok(()));

        let _ = profile_domain::port::update_profile_avatar_lock();
        let ctx = profile_domain::port::mock_update_profile_avatar::call_context();
        ctx.expect().times(0);

        let _ = bucket_domain::port::get_bucket_object_url_lock();
        let ctx = bucket_domain::port::mock_get_bucket_object_url::call_context();
        ctx.expect().times(0);

        let repo = OnAvatarCreatedRepository {
            get_bucket_object_info: bucket_domain::port::mock_get_bucket_object_info::call,
            delete_bucket_object: bucket_domain::port::mock_delete_bucket_object::call,
            update_profile_avatar: profile_domain::port::mock_update_profile_avatar::call,
            get_bucket_object_url: bucket_domain::port::mock_get_bucket_object_url::call,
        };

        let result = on_avatar_created(key, repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn success() {
        let key = "profile/avatar/1".to_string();

        let _ = bucket_domain::port::get_bucket_object_info_lock();
        let ctx = bucket_domain::port::mock_get_bucket_object_info::call_context();
        let object_head = BucketObjectHead {
            mime: "image/png".to_string(),
            ..Default::default()
        };
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .return_once(move |_| Ok(object_head));

        let _ = bucket_domain::port::delete_bucket_object_lock();
        let ctx = bucket_domain::port::mock_delete_bucket_object::call_context();
        ctx.expect().times(0);

        let _ = profile_domain::port::update_profile_avatar_lock();
        let ctx = profile_domain::port::mock_update_profile_avatar::call_context();
        ctx.expect()
            .withf(|id, url| id == "1" && url.unwrap() == "https://example.com")
            .times(1)
            .returning(|_, _| Ok(()));
        ctx.expect()
            .withf(|id, url| id == "1" && url.is_none())
            .times(0);

        let _ = bucket_domain::port::get_bucket_object_url_lock();
        let ctx = bucket_domain::port::mock_get_bucket_object_url::call_context();
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .return_once(move |_| Ok("https://example.com".to_string()));

        let repo = OnAvatarCreatedRepository {
            get_bucket_object_info: bucket_domain::port::mock_get_bucket_object_info::call,
            delete_bucket_object: bucket_domain::port::mock_delete_bucket_object::call,
            update_profile_avatar: profile_domain::port::mock_update_profile_avatar::call,
            get_bucket_object_url: bucket_domain::port::mock_get_bucket_object_url::call,
        };

        let result = on_avatar_created(key, repo).await;

        assert!(result.is_ok());
    }
}
