use profile_domain::model::create_profile_params::CreateProfileParams;
use serde::Deserialize;
use with_id::WithId;

#[derive(WithId, Clone, Deserialize, PartialEq, Eq, Debug)]
pub struct CreateProfileDto {
    pub name: String,
}

impl From<CreateProfileDtoWithId> for CreateProfileParams {
    fn from(CreateProfileDtoWithId(id, dto): CreateProfileDtoWithId) -> Self {
        CreateProfileParams { id, name: dto.name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let dto = CreateProfileDto {
            name: "name".to_string(),
        };
        let dto_with_id = CreateProfileDtoWithId("id".to_string(), dto);
        let params = CreateProfileParams::from(dto_with_id);
        assert_eq!(params.id, "id");
        assert_eq!(params.name, "name");
    }
}
