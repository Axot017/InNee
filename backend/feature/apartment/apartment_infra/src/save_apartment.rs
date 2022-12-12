use apartment_domain::model::apartment_creation::ApartmentCreation;
use common_domain::error::Result;

pub async fn save_apartment(_apartment_creation: &ApartmentCreation) -> Result<()> {
    Ok(())
}
