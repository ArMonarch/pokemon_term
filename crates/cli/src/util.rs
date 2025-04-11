use anyhow::Context;
use std::fs::File;
use std::io::Read;

use crate::pokemon::Pokemon;

/// function to append &str to (&mut String) buffer insted of using macro ->  write!(buffer, formatted_string);
pub fn write(string: &mut String, formatted_string: &str) {
    string.push_str(&formatted_string);
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

pub fn load_pokemon_sprite<'a>(
    pokemon: &String,
    _pokemon_form: &Option<String>,
    shiny: bool,
) -> anyhow::Result<Vec<u8>> {
    use std::fs::File;

    let pokemon_sprit_file_path = if shiny {
        format!("assets/colorscripts/shiny/{}", pokemon)
    } else {
        format!("assets/colorscripts/regular/{}", pokemon)
    };

    let mut buffer = Vec::new();
    File::open(&pokemon_sprit_file_path)?
        .read_to_end(&mut buffer)
        .with_context(|| format!("file {} Not Found.", pokemon_sprit_file_path))?;

    Ok(buffer)
}

pub fn load_pokemon_art(pokemon_art_path: &String) -> anyhow::Result<Vec<u8>> {
    use std::fs::File;

    let mut buffer = Vec::new();
    File::open(pokemon_art_path)?
        .read_to_end(&mut buffer)
        .with_context(|| format!("file {} Not Found.", pokemon_art_path))?;

    Ok(buffer)
}
