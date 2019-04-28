extern crate reqwest;
use std::error;
use std::io::{self, Read, Write};

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

fn fetch(query: &String) -> Result<String, Box<error::Error>> {
	let url = format!("https://pokeapi.co/api/v2/pokemon/{}", query);
	let mut response = reqwest::get(&url)?;
	let mut body = String::new();
	response.read_to_string(&mut body)?;
	Ok(body)
}

fn main() {
	let query = prompt();
	println!("{:?}", fetch(&query));
}
