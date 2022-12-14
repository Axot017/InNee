#[derive(Clone, PartialEq, Debug)]
pub struct CreateApartmentParams {
    pub name: String,
    pub description: Option<String>,
    pub price: f32,
}
