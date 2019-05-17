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
    id: u32,
    name: String,
    weight: u32,
    height: u32,
    pub abilities: Vec<Ability>,
}

impl Pokemon {
    pub fn new(name: &str) -> Result<Self, reqwest::Error> {
        let response = api::Pokemon::get_by_name(name)?;
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
            Ok(Self::fetch(&format!("pokemon/{}", name))?)
        }

        fn fetch(url: &str) -> Result<Self, Error> {
            Ok(Client::new()
                .get(&format!("https://pokeapi.co/api/v2/{}", url))
                .send()?
                .json()?)
        }
    }
}
