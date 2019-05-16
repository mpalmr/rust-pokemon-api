use crate::api;

pub struct Ability {
    pub name: String,
}

impl Ability {
    pub fn new(ability: api::Ability) -> Self {
        Self { name: ability.name }
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
                .map(|wrapper| Ability::new(wrapper.ability))
                .collect(),
        })
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
        self.abilities.iter().for_each(|ability| println!("{}", ability.name));
    }

    /// Placeholder
    pub fn show_ability(&self, ability_name: &str) {
        println!("{}", ability_name);
    }
}
