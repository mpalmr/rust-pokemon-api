#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod pokemon;

use crate::pokemon::Pokemon;
use std::io::{self, Write};
use std::str::FromStr;

enum MoreDetailsOption {
    Exit,
    Abilities,
    SearchPokemon,
}

fn main() {
    loop {
        let pokemon_name = &name_prompt();
        if pokemon_name == "q" {
            break;
        }

        if let Ok(pokemon) = Pokemon::new(pokemon_name) {
            println!("\n{}", pokemon);
            match more_details_prompt() {
                MoreDetailsOption::Exit => break,
                MoreDetailsOption::SearchPokemon => println!("\n"),
                MoreDetailsOption::Abilities => {
                    if let Some(builder) = ability_prompt(&pokemon) {
                        if let Ok(ability) = pokemon.fetch_ability(&builder.name) {
                            println!("ability: {}", ability.name);
                        } else {
                            println!("fetch_ability failed");
                        };
                    } else {
                        println!("ability_prompt failed");
                    }
                }
            }
        } else {
            println!("Could not find a pokemon by the name {}.\n", pokemon_name);
        }
    }
}

fn name_prompt() -> String {
    loop {
        let mut input = String::new();
        print!("Name a pokemon or \"q\" to quit: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        if !input.is_empty() {
            return input;
        }
    }
}

fn more_details_prompt() -> MoreDetailsOption {
    println!("What would you like to know more about?");
    println!("[1] Abilities");
    println!("[2] Search for new pokemon");
    println!("[0] Exit\n");
    loop {
        match prompt_option::<u8>() {
            Ok(0) => return MoreDetailsOption::Exit,
            Ok(1) => return MoreDetailsOption::Abilities,
            Ok(2) => return MoreDetailsOption::SearchPokemon,
            _ => println!("Invalid option.\n"),
        }
    }
}

fn ability_prompt(pokemon: &Pokemon) -> Option<&pokemon::builder::Ability> {
    loop {
        println!("\nPick one from the following abilities:");
        pokemon
            .abilities
            .iter()
            .enumerate()
            .for_each(|(i, ability)| {
                println!("[{}] {}", (i + 1).to_string(), ability.name);
            });
        println!("[0] Go Back\n");

        if let Ok(option) = prompt_option::<usize>() {
            if option == 0 {
                return None;
            }
            if let Some(ability) = pokemon.abilities.get(option - 1) {
                return Some(ability);
            }
        }
        println!("Invalid option\n");
    }
}

fn prompt_option<T: FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();
    print!("Option: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    Ok(input.trim().parse::<T>()?)
}
