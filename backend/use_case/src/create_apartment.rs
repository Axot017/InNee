use apartment_domain::{model::apartment_creation::ApartmentCreation, port::SaveApartment};
use common_domain::error::Result;

pub struct CreateApartmentRepository<T> {
    pub save: T,
}

pub async fn create_apartment<T>(
    _apartment_creation: ApartmentCreation,
    _repository: CreateApartmentRepository<T>,
) -> Result<()>
where
    for<'a> T: SaveApartment<'a>,
{
    Ok(())
}
