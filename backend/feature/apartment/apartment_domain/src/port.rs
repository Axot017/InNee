use crate::model::CreateApartmentParams;
use common_domain::define_port;
use common_domain::error::Result;

define_port!(SaveApartment = FnOnce(create_apartment_params: CreateApartmentParams) -> Result<()>);
