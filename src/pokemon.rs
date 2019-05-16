use crate::api;

pub struct Ability {
    pub name: String,
}

pub struct Pokemon {
    id: u32,
    name: String,
    weight: u32,
    height: u32,
    pub abilities: Vec<Ability>,
}

impl Pokemon {
    pub fn new(response: api::Pokemon) -> Self {
        Self {
            id: response.id,
            name: response.name,
            weight: response.weight,
            height: response.height,
            abilities: response
                .abilities
                .into_iter()
                .map(|a| Ability {
                    name: a.ability.name,
                })
                .collect(),
        }
    }

    pub fn show(&self) {
        println!("\n\nBasic Info\n==========");
        println!(
            "id: {id}\nname: {name}\nweight: {weight}\nheight: {height}\n",
            id = self.id,
            name = self.name,
            weight = self.weight,
            height = self.height,
        );
        println!("\nAbilities\n=========");
        for ability in &self.abilities {
            println!("{}", ability.name);
        }
    }

    pub fn show_ability(&self, ability_name: &str) {
        println!("{}", ability_name);
    }
}

pub fn get_by_name(name: &str) -> Result<Pokemon, reqwest::Error> {
    Ok(Pokemon::new(api::Pokemon::get_by_name(name)?))
}
