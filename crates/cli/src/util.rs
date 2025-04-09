use anyhow::Context;
use std::fs::File;
use std::io::Read;

use crate::pokemon::Pokemon;

/// function to append &str to (&mut String) buffer insted of using macro ->  write!(buffer, formatted_string);
pub fn write(string: &mut String, formatted_string: &str) {
    string.push_str(&formatted_string);
}

// function to load pokemon.json "assets/pokemon.json" as `Pokemon` struct with serde_json.
pub fn load_pokemon_json<'a>() -> anyhow::Result<Vec<Pokemon>> {
    const POKEMON_JSON_PATH: &str = "assets/pokemon.json";

    let mut buffer = String::new();

    File::open(POKEMON_JSON_PATH)
        .with_context(|| format!("assets/pokemon.json not found.\nmake sure assets directory is present along side the binary.\n"))?
        .read_to_string(&mut buffer)?;

    let pokemon: Vec<Pokemon> = serde_json::from_str(&buffer)?;

    Ok(pokemon)
}

pub fn format_command_list_output(pokemons: &[Pokemon]) -> String {
    // Const that determine the no of pokemon name to be printed per line.
    const NO_OF_COLUMNS: u8 = 4;
    // Const that determine the padding between pokemon names per line limiting each line length to
    // 80.
    const COLUMN_SIZE: u8 = 80 / NO_OF_COLUMNS;

    let mut result = String::new();
    for (i, pokemon) in pokemons.iter().enumerate() {
        if i > 0 && i % (NO_OF_COLUMNS as usize) == 0 {
            write(&mut result, "\n");
        }

        let pokemon_name = pokemon.name.get("en").unwrap();

        write(&mut result, pokemon_name);
        write(
            &mut result,
            &" ".repeat(COLUMN_SIZE as usize - pokemon_name.len()),
        );
    }

    result
}
