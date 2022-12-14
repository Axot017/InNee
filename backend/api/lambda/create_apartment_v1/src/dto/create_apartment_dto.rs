use apartment_domain::model::CreateApartmentParams;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, PartialEq, Debug)]
pub struct CreateApartmentDto {
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(length(max = 2048))]
    pub description: Option<String>,
    #[validate(range(min = 0.0))]
    pub price: f32,
}

impl From<CreateApartmentDto> for CreateApartmentParams {
    fn from(dto: CreateApartmentDto) -> Self {
        Self {
            name: dto.name,
            description: dto.description,
            price: dto.price,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_dto_to_model() {
        let dto = CreateApartmentDto {
            name: "name".to_owned(),
            description: Some("description".to_owned()),
            price: 1.0,
        };

        let model = CreateApartmentParams::from(dto);

        assert_eq!(model.name, "name");
        assert_eq!(model.description, Some("description".to_owned()));
        assert_eq!(model.price, 1.0);
    }
}
