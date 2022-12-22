use apartment_domain::model::Apartment;
use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::{config::CONFIG, dynamodb_client::get_dynamodb_client};
use serde_dynamo::from_items;

use crate::dto::apartment_dto::ApartmentDto;

pub async fn get_user_apartments(user_id: &str) -> Result<Vec<Apartment>> {
    let client = get_dynamodb_client().await;
    client
        .query()
        .table_name(&CONFIG.dynamodb_table)
        .key_condition_expression("PK = :pk and begins_with(SK, :sk)")
        .expression_attribute_values(":pk", AttributeValue::S(user_id.to_owned()))
        .expression_attribute_values(":sk", AttributeValue::S("apartment-".to_owned()))
        .send()
        .await
        .map(|result| result.items.unwrap_or_default())
        .map_err(|e| {
            Error::unknown(format!(
                "Failed to get user ({user_id:?}) apartments: {e:?}"
            ))
        })
        .and_then(|items| {
            from_items::<_, ApartmentDto>(items)
                .map_err(|e| Error::unknown(format!("Failed to format items: {e:?}")))
        })
        .map(|apartments| {
            apartments
                .into_iter()
                .map(|i| i.into())
                .collect::<Vec<Apartment>>()
        })
}
