use apartment_domain::{model::CreateApartmentParams, port::SaveApartment};
use common_domain::error::Result;

pub struct CreateApartmentRepository<T> {
    pub save: T,
}

pub async fn create_apartment<T>(
    create_apartment_params: CreateApartmentParams,
    repository: CreateApartmentRepository<T>,
) -> Result<()>
where
    T: SaveApartment,
{
    (repository.save)(create_apartment_params).await
}
