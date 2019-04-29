use crate::pokemon::Pokemon;
use std::io::{self, Write};

pub fn name() -> String {
    let mut input = String::new();
    print!("Name a pokemon: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

pub enum MoreDetailsOption {
    Abilities,
    SearchPokemon,
}

pub fn more_details() -> MoreDetailsOption {
    print!("\n\nWhat would you like to know more about?\n");
    print!("[1] Abilities\n");
    print!("[2] Search for new pokemon\n\n");
    loop {
        let mut input = String::new();
        print!("Option: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u8>().unwrap() {
            1 => return MoreDetailsOption::Abilities,
            2 => return MoreDetailsOption::SearchPokemon,
            _ => {
                print!("Invalid option.\n");
                continue;
            }
        }
    }
}

pub fn ability_name(pokemon: &Pokemon) -> &String {
    print!("Pick one from the following abilities:\n\n");
    for (i, ability) in pokemon.abilities.iter().enumerate() {
        print!("[{}] {}\n", i, ability.name);
    }
    loop {
        let mut input = String::new();
        print!("Option: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let i = input.trim().parse::<usize>().unwrap();
        if i >= pokemon.abilities.len() {
            print!("Invalid option.\n");
        } else {
            return &pokemon.abilities[i].name;
        }
    }
}
