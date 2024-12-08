use serde::Deserialize;

#[derive(Deserialize)]
pub struct IPAdd {
    pub from: String,
    pub key: String,
}

#[derive(Deserialize)]
pub struct IPSub {
    pub from: String,
    pub to: String,
}

#[derive(Deserialize)]
pub struct IPAddV6Query {
    pub from: String,
    pub key: String,
}

#[derive(Deserialize)]
pub struct IPSubV6Query {
    pub from: String,
    pub to: String,
}
