use apartment_domain::model::Apartment;
use serde::Serialize;

#[derive(Serialize, PartialEq, Debug)]
pub struct ApartmentDto {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub price: f32,
    pub description: Option<String>,
}

impl From<Apartment> for ApartmentDto {
    fn from(apartment: Apartment) -> Self {
        Self {
            id: apartment.id,
            user_id: apartment.user_id,
            name: apartment.name,
            price: apartment.price,
            description: apartment.description,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_apartment() {
        let apartment = Apartment {
            id: "id".to_string(),
            user_id: "user_id".to_string(),
            name: "name".to_string(),
            price: 1.0,
            description: Some("description".to_string()),
        };

        let apartment_dto = ApartmentDto::from(apartment);

        assert_eq!(
            apartment_dto,
            ApartmentDto {
                id: "id".to_string(),
                user_id: "user_id".to_string(),
                name: "name".to_string(),
                price: 1.0,
                description: Some("description".to_string()),
            }
        );
    }
}
