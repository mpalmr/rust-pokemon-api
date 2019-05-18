use std::fmt;

pub struct Ability {
    pub name: String,
}

impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Ability\n=======")?;
        writeln!(f, "Name: {}", self.name)
    }
}

pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub weight: u32,
    pub height: u32,
    pub abilities: Vec<Ability>,
}

impl Pokemon {
    pub fn new(name: &str) -> Result<Self, reqwest::Error> {
        let response = api::fetch_pokemon(name)?;
        Ok(Self {
            id: response.id,
            name: response.name,
            weight: response.weight,
            height: response.height,
            abilities: response
                .abilities
                .into_iter()
                .map(|wrapper| Ability {
                    name: wrapper.ability.name,
                })
                .collect(),
        })
    }
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Basic Info\n==========")?;
        writeln!(f, "id: {}", self.id)?;
        writeln!(f, "name: {}", self.name)?;
        writeln!(f, "weight: {}", self.weight)?;
        writeln!(f, "height: {}", self.height)?;
        writeln!(f, "\nAbilities\n=========")?;
        self.abilities
            .iter()
            .try_for_each(|ability| writeln!(f, "{}", ability.name))
    }
}

mod api {
    use reqwest::{Client, Error};
    use serde::Deserialize;

    const BASE_URL: &str = "https://pokeapi.co/api/v2/";

    fn fetch<T>(url: &str) -> Result<T, Error> {
        Ok(Client::new()
            .get(&format!("https://pokeapi.co/api/v2/{}", url))
            .send()?
            .json()?)
    }

    #[derive(Deserialize)]
    pub struct PokemonResponse {
        pub id: u32,
        pub name: String,
        pub weight: u32,
        pub height: u32,
        pub abilities: Vec<PokemonResponseAbilityWrapper>,
    }

    #[derive(Deserialize)]
    pub struct PokemonResponseAbilityWrapper {
        pub ability: PokemonResponseAbility,
        pub is_hidden: bool,
    }

    #[derive(Deserialize)]
    pub struct PokemonResponseAbility {
        pub id: u32,
        pub name: String,
        pub url: String,
    }

    pub fn fetch_pokemon(name: &str) -> Result<PokemonResponse, Error> {
        Ok(Client::new()
            .get(&format!("{}pokemon/{}", BASE_URL, name))
            .send()?
            .json()?)
    }

    #[derive(Deserialize)]
    pub struct AbilityResponse {
        pub id: u32,
        pub name: String,
        pub is_main_series: bool,
    }

    pub fn fetch_ability(id: u32) -> Result<AbilityResponse, Error> {
        Ok(Client::new()
            .get(&format!("{}ability/{}", BASE_URL, id))
            .send()?
            .json()?)
    }
}
