use aws_lambda_events::s3;
use lambda_runtime::{Error, LambdaEvent};
use use_case::on_avatar_created::{on_avatar_created, OnAvatarCreatedRepository};

pub async fn handle_event(event: LambdaEvent<s3::S3Event>) -> Result<(), Error> {
    for record in event.payload.records {
        let key = record.s3.object.key.ok_or("Key is empty")?;

        on_avatar_created(
            key,
            OnAvatarCreatedRepository {
                get_bucket_object_info: bucket_infra::repository::get_s3_object_info,
                delete_bucket_object: bucket_infra::repository::delete_s3_object,
                update_profile_avatar: profile_infra::repository::update_profile_avatar,
                get_bucket_object_url: bucket_infra::repository::get_s3_object_url,
            },
        )
        .await
        .map_err(Box::new)?;
    }

    Ok(())
}
