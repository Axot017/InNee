use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::{config::CONFIG, dynamodb_client::get_dynamodb_client};
use profile_domain::model::profile::Profile;
use serde_dynamo::from_item;

use crate::{dto::profile_dto::ProfileDto, PROFILE_ID_PREFIX, PROFILE_SORT_KEY};

pub async fn get_profile_by_id(id: &str) -> Result<Option<Profile>> {
    let client = get_dynamodb_client().await;
    client
        .get_item()
        .table_name(&CONFIG.dynamodb_table)
        .key(
            "PK",
            AttributeValue::S(format!("{}{}", PROFILE_ID_PREFIX, id)),
        )
        .key("SK", AttributeValue::S(PROFILE_SORT_KEY.to_owned()))
        .send()
        .await
        .map(|result| result.item)
        .map_err(|e| Error::unknown(format!("Failed to get profile ({id:?}): {e:?}")))
        .and_then(|item| match item {
            Some(item) => from_item::<_, ProfileDto>(item)
                .map(Profile::from)
                .map(Some)
                .map_err(|e| Error::unknown(format!("Failed to format item: {e:?}"))),
            None => Ok(None),
        })
}
