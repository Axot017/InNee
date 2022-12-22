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

#[cfg(test)]
mod test {
    use common_domain::tokio;
    use mockall::predicate;

    use super::*;

    #[tokio::test]
    async fn should_create_apartment() {
        let create_apartment_params = CreateApartmentParams {
            name: "apartment-1".to_string(),
            description: Some("description".to_string()),
            price: 100.0,
        };

        let _get_user_apartments_lock = apartment_domain::port::get_user_apartments_lock().await;
        let ctx = apartment_domain::port::mock_get_user_apartments::call_context();
        ctx.expect()
            .times(1)
            .withf(|user_id| user_id == "user-1")
            .returning(|_| Ok(vec![]));

        let _save_apartment_lock = apartment_domain::port::save_apartment_lock().await;
        let ctx = apartment_domain::port::mock_save_apartment::call_context();
        ctx.expect()
            .times(1)
            .with(predicate::eq(create_apartment_params.clone()))
            .returning(|_| Ok(()));

        let repository = CreateApartmentRepository {
            save_apartment: apartment_domain::port::mock_save_apartment::call,
            get_user_apartments: apartment_domain::port::mock_get_user_apartments::call,
        };

        let result = create_apartment(create_apartment_params.clone(), repository).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn apartment_already_exists() {
        let create_apartment_params = CreateApartmentParams {
            name: "apartment-1".to_string(),
            description: Some("description".to_string()),
            price: 100.0,
        };

        let _get_user_apartments_lock = apartment_domain::port::get_user_apartments_lock().await;
        let ctx = apartment_domain::port::mock_get_user_apartments::call_context();
        ctx.expect()
            .times(1)
            .withf(|user_id| user_id == "user-1")
            .returning(|_| {
                Ok(vec![apartment_domain::model::Apartment {
                    id: "apartment-1".to_string(),
                    name: "apartment-1".to_string(),
                    user_id: "user-1".to_string(),
                    description: Some("description".to_string()),
                    price: 100.0,
                }])
            });

        let repository = CreateApartmentRepository {
            save_apartment: apartment_domain::port::mock_save_apartment::call,
            get_user_apartments: apartment_domain::port::mock_get_user_apartments::call,
        };

        let result = create_apartment(create_apartment_params.clone(), repository).await;

        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().error_type,
            common_domain::error::ErrorType::Conflict
        );
        assert_eq!(result.unwrap_err(), apartment_already_exists_error());
    }
}
