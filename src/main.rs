#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod api;
mod pokemon;
mod prompt;

use crate::pokemon::Pokemon;
use prompt::MoreDetailsOption;

fn main() {
    loop {
        let pokemon_name = &prompt::name();
        if pokemon_name == "q" {
            break;
        }
        let pokemon = Pokemon::new(pokemon_name).expect("Could not retrieve pokemon");

        pokemon.show();
        match prompt::more_details() {
            MoreDetailsOption::Exit => break,
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
