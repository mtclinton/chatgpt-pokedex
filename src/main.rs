use std::io::{stdout, Write};
use termion::{clear, color, cursor};
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Pokemon {
    id: u32,
    name: String,
    height: u32,
    weight: u32,
}

fn main() {
    let mut stdout = stdout();
    write!(
        stdout,
        "{}{}{}Welcome to the Rust Pokedex!{}",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Yellow),
        color::Fg(color::Reset)
    )
    .unwrap();

    loop {
        write!(stdout, "{}", cursor::Goto(1, 3)).unwrap();
        stdout.flush().unwrap();

        print!("Enter a Pokemon name: ");
        stdout.flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let pokemon_name = input.trim().to_lowercase();

        if pokemon_name.is_empty() {
            break;
        }

        let api_url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name);
        let response = get(&api_url).unwrap().json::<Pokemon>().unwrap();

        write!(
            stdout,
            "{}{}Name:{} {}\nHeight:{}\nWeight:{}\n",
            clear::CurrentLine,
            cursor::Goto(1, 4),
            response.name,
            color::Fg(color::Green),
            response.height,
            response.weight,
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}
