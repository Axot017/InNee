use profile_domain::model::profile::Profile;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct ProfileDto {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

impl From<Profile> for ProfileDto {
    fn from(profile: Profile) -> Self {
        Self {
            id: profile.id,
            name: profile.name,
            avatar_url: profile.avatar_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_profile() {
        let profile = Profile {
            id: "id".to_string(),
            name: "John Doe".to_owned(),
            avatar_url: Some("https://example.com/avatar.png".to_owned()),
        };
        let profile_dto = ProfileDto::from(profile);
        assert_eq!(
            profile_dto,
            ProfileDto {
                id: "id".to_string(),
                name: "John Doe".to_owned(),
                avatar_url: Some("https://example.com/avatar.png".to_owned()),
            }
        );
    }
}
