use anyhow::Context;
use std::io::Read;

use crate::pokemon::Pokemon;

/// function to append &str to (&mut String) buffer insted of using macro ->  write!(buffer, formatted_string);
pub fn write(string: &mut String, formatted_string: &str) {
    string.push_str(&formatted_string);
}

pub fn format_command_list_output(pokes: &[Pokemon], args: &crate::args::Args) -> String {
    // Var that determine the no of pokemon name to be printed per line.
    //
    // Default: 4 Columns for listing only Pokemons OR 1 Columns for listing Pokemons with Forms.
    #[allow(non_snake_case)]
    let NO_OF_COLUMNS: u8 = if !args.list_with_forms { 4 } else { 2 };

    // Var that determine the padding between pokemon names per line limiting each line length to
    // 80.
    #[allow(non_snake_case)]
    let COLUMN_SIZE: u8 = 80 / NO_OF_COLUMNS;

    let mut result = String::new();

    if args.list_with_forms {
        write(&mut result, "Pokemon Name: Forms\n");
    }

    for (i, poke) in pokes.iter().enumerate() {
        let mut padding: usize = 0;

        if i > 0 && i % (NO_OF_COLUMNS as usize) == 0 {
            write(&mut result, "\n");
        }

        let pokemon_name = poke.name.get("en").unwrap();

        write(&mut result, pokemon_name);
        padding += pokemon_name.len();

        if args.list_with_forms {
            write(&mut result, ":");

            let poke_forms = if poke.forms.is_empty() {
                padding += 5; // for `_N/A` (4) and `:` (1)
                String::from(" N/A")
            } else {
                padding += 1; // for `:` before printing Pokes forms.
                padding += poke.forms.iter().fold(0, |acc, x| acc + 1 + x.len()); // +1
                // for the space added between every forms.
                poke.forms
                    .iter()
                    .fold(String::new(), |acc, val| acc + " " + val)
            };
            write(&mut result, &poke_forms);
        }

        write(&mut result, &" ".repeat(COLUMN_SIZE as usize - padding));
    }

    result
}

pub fn load_pokemon_art(pokemon_art_path: &String) -> anyhow::Result<Vec<u8>> {
    use std::fs::File;

    let mut buffer = Vec::new();
    File::open(pokemon_art_path)?
        .read_to_end(&mut buffer)
        .with_context(|| format!("file {} Not Found.", pokemon_art_path))?;

    Ok(buffer)
}
