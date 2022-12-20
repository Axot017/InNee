use apartment_domain::model::Apartment;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct ApartmentDto {
    #[serde(rename = "SK")]
    pub id: String,
    #[serde(rename = "PK")]
    pub user_id: String,
    pub name: String,
    pub price: f32,
    pub description: Option<String>,
}

impl From<ApartmentDto> for Apartment {
    fn from(dto: ApartmentDto) -> Self {
        Apartment {
            id: dto.id,
            user_id: dto.user_id,
            name: dto.name,
            price: dto.price,
            description: dto.description,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_apartment_dto_to_apartment() {
        let dto = ApartmentDto {
            id: "id".to_string(),
            user_id: "user_id".to_string(),
            name: "name".to_string(),
            price: 1.0,
            description: Some("description".to_string()),
        };

        let apartment = Apartment::from(dto);

        assert_eq!(apartment.id, "id");
        assert_eq!(apartment.user_id, "user_id");
        assert_eq!(apartment.name, "name");
        assert_eq!(apartment.price, 1.0);
        assert_eq!(apartment.description, Some("description".to_string()));
    }
}
