#[derive(PartialEq, Debug)]
pub struct ApartmentCreation {
    pub name: String,
    pub description: Option<String>,
    pub price: f32,
}
