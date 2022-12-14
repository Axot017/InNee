use apartment_domain::model::CreateApartmentParams;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct CreateApartmentDto {
    #[serde(rename = "PK")]
    pub profile_id: String,
    #[serde(rename = "SK")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f32,
}

impl From<CreateApartmentParams> for CreateApartmentDto {
    fn from(create_apartment_params: CreateApartmentParams) -> Self {
        // TODO: Use real profile_id
        let profile_uuid = Uuid::new_v4();
        let apartment_id = Uuid::new_v4();
        CreateApartmentDto {
            id: format!("apartment-{}", apartment_id.simple()),
            profile_id: format!("profile-{}", profile_uuid.simple()),
            name: create_apartment_params.name,
            description: create_apartment_params.description,
            price: create_apartment_params.price,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_create_apartment_params() {
        let create_apartment_params = CreateApartmentParams {
            name: "Test".to_string(),
            description: Some("Test description".to_string()),
            price: 100.0,
        };
        let create_apartment_dto = CreateApartmentDto::from(create_apartment_params);
        assert_eq!(create_apartment_dto.name, "Test");
        assert_eq!(
            create_apartment_dto.description,
            Some("Test description".to_string())
        );
        assert_eq!(create_apartment_dto.price, 100.0);
        assert!(create_apartment_dto.profile_id.starts_with("profile-"));
        assert!(create_apartment_dto.id.starts_with("apartment-"));
    }
}
