use crate::api;
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
