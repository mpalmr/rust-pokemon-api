#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod pokemon;

use crate::pokemon::{Ability, Pokemon};
use colored::*;
use std::io::{self, Write};

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
                    if let Some(ability) = ability_prompt(&pokemon) {
                        println!("\n\n{}", ability);
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
        print!("{} ", "Name a pokemon or \"q\" to quit:".green().bold());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<String>().unwrap();
        if !input.is_empty() {
            return input;
        }
    }
}

fn more_details_prompt() -> MoreDetailsOption {
    println!(
        "\n{}",
        "What would you like to know more about?".green().bold()
    );
    println!("{} {}", "[1]".blue().bold(), "Abilities".blue());
    println!(
        "{} {}",
        "[2]".blue().bold(),
        "Search for new pokemon".blue()
    );
    println!("{} {}\n", "[0]".red().bold(), "Exit".red());

    loop {
        let mut input = String::new();
        print!("Option: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u8>().unwrap() {
            0 => return MoreDetailsOption::Exit,
            1 => return MoreDetailsOption::Abilities,
            2 => return MoreDetailsOption::SearchPokemon,
            _ => println!("{}", "Invalid option.".red()),
        }
    }
}

fn ability_prompt(pokemon: &Pokemon) -> Option<&Ability> {
    loop {
        println!(
            "\n{}",
            "Pick one from the following abilities:".green().bold()
        );
        pokemon
            .abilities
            .iter()
            .enumerate()
            .for_each(|(i, ability)| {
                println!(
                    "{} {}",
                    &format!("[{}]", (i + 1).to_string()).blue().bold(),
                    ability.name.blue(),
                )
            });
        println!("{} {}\n", "[0]".red().bold(), "Go Back".red());

        let mut input = String::new();
        print!("Option: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<usize>().unwrap();

        if input == 0 {
            return None;
        }
        if let Some(ability) = pokemon.abilities.get(input - 1) {
            return Some(&ability);
        }
        println!("{}", "Invalid option.".red());
    }
}
