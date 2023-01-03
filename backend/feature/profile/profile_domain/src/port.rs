use crate::model::create_profile_params::CreateProfileParams;
use crate::model::profile::Profile;
use common_domain::define_port;
use common_domain::error::Result;

define_port!(GetProfileById = FnOnce<'a>(id: &'a str) -> Result<Option<Profile>>);

define_port!(SaveProfile = FnOnce(params: CreateProfileParams) -> Result<()>);
