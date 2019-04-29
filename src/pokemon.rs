use reqwest::Error;

mod api {
    use reqwest::{Error, Client};
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
        pub fn get_by_name(name: &str) -> Result<Pokemon, Error> {
            Ok(Pokemon::fetch(&format!("https://pokeapi.co/api/v2/pokemon/{}", name))?)
        }

        fn fetch(url: &str) -> Result<Pokemon, Error> {
            Ok(Client::new()
                .get(url)
                .send()?
                .json()?)
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
    pub fn new(response: api::Pokemon) -> Pokemon {
        let mut abilities: Vec<Ability> = vec![];
        for ability in response.abilities.into_iter() {
            abilities.push(Ability {
                name: String::from(ability.ability.name),
            });
        }

        Pokemon {
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
        for ability in self.abilities.iter() {
            println!("{}", ability.name);
        }
    }

    pub fn show_ability(&self, ability_name: &String) {
        println!("{}", ability_name);
    }
}

pub fn get_by_name(name: &str) -> Result<Pokemon, Error> {
    Ok(Pokemon::new(api::Pokemon::get_by_name(name)?))
}
