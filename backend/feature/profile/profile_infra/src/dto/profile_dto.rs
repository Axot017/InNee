use profile_domain::model::profile::Profile;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct ProfileDto {
    pub name: String,
    pub avatar_url: Option<String>,
}

impl From<ProfileDto> for Profile {
    fn from(dto: ProfileDto) -> Self {
        Profile {
            name: dto.name,
            avatar_url: dto.avatar_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let dto = ProfileDto {
            name: "name".to_string(),
            avatar_url: Some("avatar_url".to_string()),
        };

        assert_eq!(
            Profile::from(dto),
            Profile {
                name: "name".to_string(),
                avatar_url: Some("avatar_url".to_string()),
            },
        );
    }
}
