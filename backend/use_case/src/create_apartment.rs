use std::collections::HashMap;

use apartment_domain::{
    model::CreateApartmentParams,
    port::{GetUserApartments, SaveApartment},
};
use common_domain::error::{Error, ErrorOutput, Result};

pub struct CreateApartmentRepository<T, Y> {
    pub save_apartment: T,
    pub get_user_apartments: Y,
}

pub async fn create_apartment<T, Y>(
    create_apartment_params: CreateApartmentParams,
    repository: CreateApartmentRepository<T, Y>,
) -> Result<()>
where
    T: SaveApartment,
    for<'a> Y: GetUserApartments<'a>,
{
    let existing_apartments = (repository.get_user_apartments)("user-1").await?;
    if !existing_apartments.is_empty() {
        return Err(apartment_already_exists_error());
    }
    (repository.save_apartment)(create_apartment_params).await
}

fn apartment_already_exists_error() -> Error {
    Error {
        debug_message: "User already has an apartment".to_string(),
        error_type: common_domain::error::ErrorType::Conflict,
        output: Box::new(ErrorOutput {
            message: "User already has an apartment".to_string(),
            code: "apartment_already_exists".to_string(),
            args: HashMap::new(),
        }),
    }
}
