#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod api;
mod pokemon;
mod prompt;

use prompt::MoreDetailsOption;

fn main() {
    let query = prompt::name();
    let pokemon = pokemon::get_by_name(&query).expect("Could not retrieve pokemon");
    pokemon.show();
    match prompt::more_details() {
        MoreDetailsOption::Abilities => pokemon.show_ability(&prompt::ability_name(&pokemon)),
        MoreDetailsOption::SearchPokemon => {
            println!("\n");
            main()
        }
    };
}
