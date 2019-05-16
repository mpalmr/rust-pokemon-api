#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod api;
mod pokemon;
mod prompt;

use crate::pokemon::Pokemon;
use prompt::MoreDetailsOption;
use std::process;

fn main() {
    loop {
        let pokemon = Pokemon::new(&prompt::name()).expect("Could not retrieve pokemon");
        pokemon.show();

        match prompt::more_details() {
            MoreDetailsOption::Exit => process::exit(0),
            MoreDetailsOption::SearchPokemon => println!("\n"),
            MoreDetailsOption::Abilities => {
                if let Some(name) = prompt::ability_name(&pokemon) {
                    pokemon.show_ability(name);
                } else {
                    println!("\n");
                }
            }
        };
    }
}
