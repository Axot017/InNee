use profile_domain::model::profile::Profile;
use serde::Deserialize;

use crate::PROFILE_ID_PREFIX;

#[derive(Deserialize, PartialEq, Eq, Debug)]
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
}
