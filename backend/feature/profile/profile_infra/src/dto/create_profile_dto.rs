use chrono::{DateTime, Utc};
use profile_domain::model::create_profile_params::CreateProfileParams;
use serde::Serialize;

use crate::{PROFILE_ID_PREFIX, PROFILE_SORT_KEY};

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct CreateProfileDto {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<CreateProfileParams> for CreateProfileDto {
    fn from(params: CreateProfileParams) -> Self {
        Self {
            pk: format!("{}{}", PROFILE_ID_PREFIX, params.id),
            sk: PROFILE_SORT_KEY.to_owned(),
            name: params.name,
            avatar_url: None,
            updated_at: Utc::now(),
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let params = CreateProfileParams {
            id: "id".to_owned(),
            name: "name".to_owned(),
        };
        let dto = CreateProfileDto::from(params);
        assert_eq!(
            dto,
            CreateProfileDto {
                pk: format!("{}{}", PROFILE_ID_PREFIX, "id"),
                sk: PROFILE_SORT_KEY.to_owned(),
                name: "name".to_owned(),
                avatar_url: None,
                updated_at: dto.updated_at,
                created_at: dto.created_at,
            }
        );
    }
}
