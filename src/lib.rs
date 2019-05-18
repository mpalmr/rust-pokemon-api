#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod request;

pub struct Pokemon {
    pub id: u16,
    pub name: String,
    pub weight: f32,
    pub height: f32,
    pub abilities: Vec<AbilityReference>,
}

pub struct AbilityReference {
    pub name: String,
    pub url: String,
}

impl Pokemon {
    pub fn get_by_name(name: &str) -> Result<Self, reqwest::Error> {
        request::pokemon
    }
}
