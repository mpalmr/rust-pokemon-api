use crate::pokemon::Pokemon;
use std::io::{self, Write};

pub fn name() -> String {
    let mut input = String::new();
    print!("Name a pokemon: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<String>().unwrap()
}

pub enum MoreDetailsOption {
    Abilities,
    SearchPokemon,
}

pub fn more_details() -> MoreDetailsOption {
    println!("\n\nWhat would you like to know more about?");
    println!("[1] Abilities");
    println!("[2] Search for new pokemon\n");

    loop {
        let mut input = String::new();
        print!("Option: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u8>().unwrap() {
            1 => return MoreDetailsOption::Abilities,
            2 => return MoreDetailsOption::SearchPokemon,
            _ => {
                println!("Invalid option.");
                continue;
            }
        }
    }
}

pub fn ability_name(pokemon: &Pokemon) -> &str {
    println!("\nPick one from the following abilities:");
    pokemon
        .abilities
        .iter()
        .enumerate()
        .for_each(|(i, ability)| println!("[{}] {}", i + 1, ability.name));

    loop {
        let mut input = String::new();
        print!("\nOption: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let i = input.trim().parse::<usize>().unwrap() - 1;
        match pokemon.abilities.get(i) {
            None => println!("Invalid option."),
            _ => return &pokemon.abilities[i].name,
        }
    }
}
