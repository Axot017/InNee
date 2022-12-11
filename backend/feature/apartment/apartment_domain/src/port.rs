use crate::model::apartment_creation::ApartmentCreation;
use common_domain::define_port;
use common_domain::error::Result;

define_port!(SaveApartment = FnOnce<'a>(apartment_creation: &'a ApartmentCreation) -> Result<()>);
