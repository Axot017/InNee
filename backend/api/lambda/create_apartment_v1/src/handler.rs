use std::future;

use apartment_domain::model::apartment_creation::ApartmentCreation;
use common_api::{error_dto::ErrorDto, from_request::FromRequest, validate_dto::ValidateDto};
use futures::TryFutureExt;
use lambda_http::{Body, Error, Request, Response};
use use_case::create_apartment::{create_apartment, CreateApartmentRepository};

use crate::dto::ApartmentCreationDto;

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    let result = future::ready(
        ApartmentCreationDto::from_request(event)
            .and_then(|dto| dto.validate_dto().map(|_| dto))
            .map(ApartmentCreation::from),
    )
    .and_then(|apartment_creation| {
        create_apartment(
            apartment_creation,
            CreateApartmentRepository {
                save: apartment_infra::save_apartment::save_apartment,
            },
        )
    })
    .await;

    match result {
        Ok(result) => result,
        Err(error) => {
            return ErrorDto::from(error).try_into();
        }
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("hello".into())
        .map_err(Box::new)?;

    Ok(resp)
}
