use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub weight: u32,
    pub height: u32,
    pub abilities: Vec<AbilityWrapper>,
}

#[derive(Deserialize)]
pub struct Ability {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct AbilityWrapper {
    pub ability: Ability,
    pub is_hidden: bool,
}

impl Pokemon {
    pub fn get_by_name(name: &str) -> Result<Self, Error> {
        Ok(Self::fetch(&format!(
            "https://pokeapi.co/api/v2/pokemon/{}",
            name
        ))?)
    }

    fn fetch(url: &str) -> Result<Self, Error> {
        Ok(Client::new().get(url).send()?.json()?)
    }
}
