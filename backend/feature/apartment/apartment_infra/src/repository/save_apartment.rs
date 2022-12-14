use apartment_domain::model::CreateApartmentParams;
use common_domain::{error::Error, error::Result};
use common_infra::{config::CONFIG, dynamodb_client::get_dynamodb_client};
use serde_dynamo::to_item;

use crate::dto::create_apartment_dto::CreateApartmentDto;

pub async fn save_apartment(create_apartment_params: CreateApartmentParams) -> Result<()> {
    let client = get_dynamodb_client().await;

    let dto = CreateApartmentDto::from(create_apartment_params);
    let item = to_item(&dto)
        .map_err(|e| Error::unknown(format!("Failed to serialize item ({e:?}): {dto:?}")))?;

    client
        .put_item()
        .table_name(&CONFIG.dynamodb_table)
        .set_item(Some(item))
        .send()
        .await
        .map(|_| ())
        .map_err(|e| Error::unknown(format!("Failed to save item ({e:?}): {dto:?}")))
}
