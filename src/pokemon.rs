use reqwest::Error;

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
            Ok(Self::fetch(&format!(
                "https://pokeapi.co/api/v2/pokemon/{}",
                name
            ))?)
        }

        fn fetch(url: &str) -> Result<Self, Error> {
            Ok(Client::new().get(url).send()?.json()?)
        }
    }
}

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
        let mut abilities: Vec<Ability> = vec![];
        for ability in response.abilities {
            abilities.push(Ability {
                name: ability.ability.name,
            });
        }

        Self {
            id: response.id,
            name: response.name,
            weight: response.weight,
            height: response.height,
            abilities,
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

pub fn get_by_name(name: &str) -> Result<Pokemon, Error> {
    Ok(Pokemon::new(api::Pokemon::get_by_name(name)?))
}
