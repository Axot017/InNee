use serde::Deserialize;

#[derive(Deserialize)]
pub struct CargoContent {
    pub package: Package,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
}
