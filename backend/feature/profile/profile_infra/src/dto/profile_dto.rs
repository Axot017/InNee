use profile_domain::model::{create_profile_params::CreateProfileParams, profile::Profile};
use serde::{Deserialize, Serialize};

use crate::PROFILE_ID_PREFIX;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ProfileDto {
    #[serde(rename = "PK")]
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

impl From<ProfileDto> for Profile {
    fn from(dto: ProfileDto) -> Self {
        Profile {
            id: dto.id.replace(PROFILE_ID_PREFIX, ""),
            name: dto.name,
            avatar_url: dto.avatar_url,
        }
    }
}

impl From<CreateProfileParams> for ProfileDto {
    fn from(params: CreateProfileParams) -> Self {
        ProfileDto {
            id: format!("{}{}", PROFILE_ID_PREFIX, params.id),
            name: params.name,
            avatar_url: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_profile_dto() {
        let dto = ProfileDto {
            id: format!("{}id", PROFILE_ID_PREFIX),
            name: "name".to_string(),
            avatar_url: Some("avatar_url".to_string()),
        };

        assert_eq!(
            Profile::from(dto),
            Profile {
                id: "id".to_string(),
                name: "name".to_string(),
                avatar_url: Some("avatar_url".to_string()),
            },
        );
    }

    #[test]
    fn from_create_profile_params() {
        let params = CreateProfileParams {
            id: "id".to_string(),
            name: "name".to_string(),
        };

        assert_eq!(
            ProfileDto::from(params),
            ProfileDto {
                id: format!("{}id", PROFILE_ID_PREFIX),
                name: "name".to_string(),
                avatar_url: None,
            },
        );
    }
}
