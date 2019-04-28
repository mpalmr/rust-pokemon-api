extern crate reqwest;
extern crate serde;
extern crate serde_json;
use std::io::{self, Write};

#[derive(serde::Deserialize, Debug)]
struct Pokemon {
	id: u32,
}

fn prompt() -> String {
	let mut input = String::new();
	print!("Name a pokemon: ");
	io::stdout().flush().unwrap();
	io::stdin()
		.read_line(&mut input)
		.expect("Error reading from STDIN");
	match input.trim().parse() {
		Ok(result) => {
			if result == "" {
				prompt()
			} else {
				result
			}
		},
		Err(_) => prompt(),
	}
}

fn fetch(query: &String) -> Result<Pokemon, reqwest::Error> {
	Ok(reqwest::Client::new()
		.get(&format!("https://pokeapi.co/api/v2/pokemon/{}", query))
		.send()?
		.json()?)
}

fn main() {
	let query = prompt();
	println!("{:?}", fetch(&query));
}
