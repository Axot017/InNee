use crate::model::{Apartment, CreateApartmentParams};
use common_domain::define_port;
use common_domain::error::Result;

define_port!(SaveApartment = FnOnce(create_apartment_params: CreateApartmentParams) -> Result<()>);

define_port!(GetUserApartments = FnOnce<'a>(user_id: &'a str) -> Result<Vec<Apartment>>);
