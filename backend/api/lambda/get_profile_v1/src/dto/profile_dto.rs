use profile_domain::model::profile::Profile;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct ProfileDto {}

impl From<Profile> for ProfileDto {
    fn from(_profile: Profile) -> Self {
        Self {}
    }
}
