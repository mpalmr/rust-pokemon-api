use crate::Pokemon;
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;
use serde::Deserialize;

const BASE_URL: &str = "https://pokeapi.co/api/v2/";

fn fetch<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    Ok(Client::new().get(url).send()?.json()?)
}

#[derive(Deserialize)]
pub struct PokemonResponse {
    pub id: u16,
    pub name: String,
    pub weight: f32,
    pub height: f32,
    pub abilities: Vec<AbilityWrapper>,
}

#[derive(Deserialize)]
pub struct AbilityWrapper {
    pub ability: Ability,
    pub is_hidden: bool,
}

#[derive(Deserialize)]
pub struct Ability {
    pub name: String,
    pub url: String,
}

pub fn get_by_name(name: &str) -> Result<Pokemon, Error> {
    let response: PokemonResponse =
        fetch::<PokemonResponse>(&format!("{}pokemon/{}", BASE_URL, name))?;
}
