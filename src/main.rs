extern crate reqwest;
extern crate serde;
extern crate serde_json;
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Deserialize)]
struct PokemonAbility {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct PokemonAbilityWrapper {
    ability: PokemonAbility,
    is_hidden: bool,
    slot: u32,
}

#[derive(Deserialize)]
struct Pokemon {
    id: u32,
    name: String,
    weight: u32,
    height: u32,
    abilities: Vec<PokemonAbilityWrapper>,
}

impl Pokemon {
    pub fn fetch(name: &String) -> Result<Pokemon, reqwest::Error> {
        Ok(reqwest::Client::new()
            .get(&format!("https://pokeapi.co/api/v2/pokemon/{}", name))
            .send()?
            .json()?)
    }
}

fn main() {
    let query = prompt_name().expect("Could not retrieve user input");
    let pokemon = Pokemon::fetch(&query).expect("Could not retrieve pokemon");

    println!("\n\nBasic Info\n==========");
    println!(
        "id: {id}\nname: {name}\nweight: {weight}\nheight: {height}\n",
        id = pokemon.id,
        name = pokemon.name,
        weight = pokemon.weight,
        height = pokemon.height,
    );
    println!("\nAbilities\n=========");
    for ability_wrapper in pokemon.abilities.iter() {
        println!(
            "{slot}. {name}",
            slot = ability_wrapper.slot,
            name = ability_wrapper.ability.name,
        );
    }
}

fn prompt_name() -> Result<String, Box<std::error::Error>> {
    let mut input = String::new();
    print!("Name a pokemon: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}
