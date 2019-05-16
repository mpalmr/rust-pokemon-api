#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod api;
mod pokemon;
mod prompt;

use crate::pokemon::Pokemon;
use prompt::MoreDetailsOption;

fn main() {
    let pokemon = Pokemon::new(&prompt::name()).expect("Could not retrieve pokemon");
    pokemon.show();
    match prompt::more_details() {
        MoreDetailsOption::Abilities => pokemon.show_ability(&prompt::ability_name(&pokemon)),
        MoreDetailsOption::SearchPokemon => {
            println!("\n");
            main()
        }
    };
}
