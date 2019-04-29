extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod prompt;
mod pokemon;

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
        },
    };
}
