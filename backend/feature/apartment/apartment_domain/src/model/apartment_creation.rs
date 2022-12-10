#[derive(PartialEq, Debug)]
pub struct ApartmentCreation {
    name: String,
    description: Option<String>,
    price: f32,
}
