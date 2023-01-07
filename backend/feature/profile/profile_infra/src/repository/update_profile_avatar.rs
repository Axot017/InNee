use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::{config::CONFIG, dynamodb_client::get_dynamodb_client};

use crate::{PROFILE_ID_PREFIX, PROFILE_SORT_KEY};

pub async fn update_profile_avatar(id: &str, url: Option<&str>) -> Result<()> {
    let query = get_dynamodb_client()
        .await
        .update_item()
        .table_name(&CONFIG.dynamodb_table)
        .key(
            "PK",
            AttributeValue::S(format!("{}{}", PROFILE_ID_PREFIX, id)),
        )
        .key("SK", AttributeValue::S(PROFILE_SORT_KEY.to_owned()));

    let query = match url {
        Some(url) => query
            .update_expression("set avatar_url = :avatar_url")
            .expression_attribute_values(":avatar_url", AttributeValue::S(url.to_owned())),
        None => query.update_expression("remove avatar_url"),
    };

    query
        .send()
        .await
        .map(|_| ())
        .map_err(|e| Error::unknown(format!("{:?}", e)))
}
