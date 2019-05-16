use crate::pokemon::Pokemon;
use std::io::{self, Write};

pub fn name() -> String {
    let mut input = String::new();
    print!("Name a pokemon or \"q\" to quit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<String>().unwrap()
}

pub enum MoreDetailsOption {
    Exit,
    Abilities,
    SearchPokemon,
}

pub fn more_details() -> MoreDetailsOption {
    println!("\n\nWhat would you like to know more about?");
    println!("[1] Abilities");
    println!("[2] Search for new pokemon");
    println!("[0] Exit\n");

    loop {
        let mut input = String::new();
        print!("Option: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u8>().unwrap() {
            0 => return MoreDetailsOption::Exit,
            1 => return MoreDetailsOption::Abilities,
            2 => return MoreDetailsOption::SearchPokemon,
            _ => {
                println!("Invalid option.");
                continue;
            }
        }
    }
}

pub fn ability_name(pokemon: &Pokemon) -> Option<&str> {
    loop {
        println!("\nPick one from the following abilities:");
        pokemon
            .abilities
            .iter()
            .enumerate()
            .for_each(|(i, ability)| println!("[{}] {}", i + 1, ability.name));
        println!("[0] Go Back\n");

        let mut input = String::new();
        print!("Option: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<usize>().unwrap();

        if input == 0 {
            return None;
        }
        if let Some(ability) = pokemon.abilities.get(input - 1) {
            return Some(&ability.name);
        }
        println!("Invalid option.");
    }
}
