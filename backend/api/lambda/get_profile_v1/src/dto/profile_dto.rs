use profile_domain::model::profile::Profile;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct ProfileDto {
    pub name: String,
    pub avatar_url: Option<String>,
}

impl From<Profile> for ProfileDto {
    fn from(profile: Profile) -> Self {
        Self {
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
            name: "John Doe".to_owned(),
            avatar_url: Some("https://example.com/avatar.png".to_owned()),
        };
        let profile_dto = ProfileDto::from(profile);
        assert_eq!(
            profile_dto,
            ProfileDto {
                name: "John Doe".to_owned(),
                avatar_url: Some("https://example.com/avatar.png".to_owned()),
            }
        );
    }
}
