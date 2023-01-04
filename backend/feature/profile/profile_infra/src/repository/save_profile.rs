use common_domain::error::{Error, Result};
use common_infra::{config::CONFIG, dynamodb_client::get_dynamodb_client};
use profile_domain::model::create_profile_params::CreateProfileParams;
use serde_dynamo::to_item;

use crate::dto::create_profile_dto::CreateProfileDto;

pub async fn save_profile(params: CreateProfileParams) -> Result<()> {
    let client = get_dynamodb_client().await;

    let dto = CreateProfileDto::from(params);
    let item = to_item(&dto)
        .map_err(|e| Error::unknown(format!("Failed to serialize profile ({e:?}): {dto:?}")))?;

    client
        .put_item()
        .table_name(&CONFIG.dynamodb_table)
        .set_item(Some(item))
        .send()
        .await
        .map(|_| ())
        .map_err(|e| Error::unknown(format!("Failed to save profile ({e:?}): {dto:?}")))
}
