use std::fmt;

pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub weight: u32,
    pub height: u32,
    pub abilities: Vec<builder::Ability>,
}

impl Pokemon {
    pub fn new(name: &str) -> Result<Self, reqwest::Error> {
        let response: api::PokemonResponse = api::get_pokemon(name)?;
        Ok(Self {
            id: response.id,
            name: response.name,
            weight: response.weight,
            height: response.height,
            abilities: response
                .abilities
                .into_iter()
                .map(builder::Ability::new)
                .collect(),
        })
    }

    pub fn fetch_ability(&self, ability_name: &str) -> Result<Ability, &'static str> {
        match self.abilities.iter().find(|a| a.name == ability_name) {
            Some(builder) => {
                if let Ok(ability) = builder.fetch() {
                    Ok(Ability::new(ability))
                } else {
                    Err("could not fetch ability")
                }
            }
            None => Err("ability not found"),
        }
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

pub struct Ability {
    pub id: u32,
    pub name: String,
    pub effect_entries: Vec<String>,
}

impl Ability {
    pub fn new(response: api::AbilityResponse) -> Self {
        Self {
            id: response.id,
            name: response.name,
            effect_entries: response
                .effect_entries
                .into_iter()
                .map(|entry| entry.short_effect)
                .collect(),
        }
    }
}

impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n\nID: {}", self.id)?;
        writeln!(f, "Name: {}", self.name)?;
        self.effect_entries
            .iter()
            .try_for_each(|entry| writeln!(f, "{}", entry))
    }
}

pub mod builder {
    pub struct Ability {
        pub name: String,
        pub url: String,
    }

    impl Ability {
        pub fn new(response: super::api::PokemonResponseAbilityWrapper) -> Self {
            Self {
                name: response.ability.name,
                url: response.ability.url,
            }
        }

        pub fn fetch(&self) -> Result<super::api::AbilityResponse, reqwest::Error> {
            let response: super::api::AbilityResponse = super::api::fetch(&self.url)?;
            Ok(super::api::AbilityResponse {
                id: response.id,
                name: response.name,
                is_main_series: response.is_main_series,
                effect_entries: response.effect_entries,
            })
        }
    }
}

mod api {
    use reqwest::{Client, Error};
    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    const BASE_URL: &str = "https://pokeapi.co/api/v2/";

    pub fn fetch<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
        Ok(Client::new().get(url).send()?.json()?)
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
        pub name: String,
        pub url: String,
    }

    pub fn get_pokemon(name: &str) -> Result<PokemonResponse, Error> {
        Ok(fetch::<PokemonResponse>(&format!(
            "{}pokemon/{}",
            BASE_URL, name
        ))?)
    }

    #[derive(Deserialize)]
    pub struct AbilityResponse {
        pub id: u32,
        pub name: String,
        pub is_main_series: bool,
        pub effect_entries: Vec<AbilityResponseEffectEntry>,
    }

    #[derive(Deserialize)]
    pub struct AbilityResponseEffectEntry {
        pub effect: String,
        pub short_effect: String,
    }
}
