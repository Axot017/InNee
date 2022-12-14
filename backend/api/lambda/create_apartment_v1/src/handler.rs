use apartment_domain::model::CreateApartmentParams;
use common_api::{
    from_request::FromRequest, into_response::IntoResponse, validate_dto::ValidateDto,
};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::create_apartment::{create_apartment, CreateApartmentRepository};

use crate::dto::CreateApartmentDto;

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    CreateApartmentDto::from_request(event)
        .and_then(|dto| dto.validate_dto().map(|_| dto))
        .map(CreateApartmentParams::from)
        .into_future()
        .and_then(|create_apartment_params| {
            create_apartment(
                create_apartment_params,
                CreateApartmentRepository {
                    save: apartment_infra::repository::save_apartment,
                },
            )
        })
        .await
        .into_empty_response(StatusCode::CREATED)
}
