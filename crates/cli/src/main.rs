//! The main entry point for pokemon-term

use std::process::ExitCode;

mod args;
mod flags;
mod parse;
mod pokemon;

mod util;
use rand::Rng;

use crate::util::format_command_list_output;
use crate::util::load_pokemon_art;

mod help;
mod version;

fn main() -> ExitCode {
    let args = flags::parse();

    match run(args) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {:#}", err);
            return ExitCode::from(1);
        }
    }
}

/// The main entry point for pokemon-term.
fn run(args: parse::ParseResult<args::Args>) -> anyhow::Result<ExitCode> {
    use args::Mode;
    use parse::ParseResult;

    let args = match args {
        ParseResult::Ok(args) => args,
        ParseResult::Err(err) => return Err(err),
        ParseResult::Special(mode) => return special(mode),
    };

    return match args.mode {
        Mode::List => list_pokemons(args),
        Mode::Regular => print_pokemon(args),
        Mode::Random => print_random_pokemon(args),
        Mode::RandomByNames => print_random_pokemon_by_name(args),
        Mode::RandomByGen => print_random_pokemon_by_gen(args),
    };
}

/// Implements pokemon-term's "special" modes.
///
/// A special mode is one that generally short-circuits most of the pokemon-term's logic and skips
/// right to this routine. The special mode essentially consists of printing help and version
/// output. The idea behind the short circuiting is to ensure there is as little as possible overhead for emiting help/version output.
fn special(mode: crate::args::SpecialMode) -> anyhow::Result<ExitCode> {
    use std::io::Write;

    use crate::args::SpecialMode::{HelpLong, HelpShort, VersionLong, VersionShort};

    use help::{help_long, help_short};
    use version::{version_long, version_short};

    let exit = ExitCode::from(0);
    let output = match mode {
        HelpShort => help_short(),
        HelpLong => help_long(),
        VersionShort => version_short(),
        VersionLong => version_long(),
    };

    writeln!(std::io::stdout(), "{}", output)?;

    Ok(exit)
}

use crate::pokemon::Pokemons;

/// type allias for Vec<Pokemon>, Because why not.

/// Top level entry point for listing all pokemons
///
/// This function parse the assets/pokemons.json to get the list of available pokemons available and prints
/// the list to the terminal.
fn list_pokemons(args: crate::args::Args) -> anyhow::Result<ExitCode> {
    let poke = Pokemons::load_json()?;

    let list_output = format_command_list_output(poke.get_all(), &args);

    println!("{}", list_output);

    let exit_code = ExitCode::from(0);
    Ok(exit_code)
}

/// Top level entry point for printing pokemon to the terminal
fn print_pokemon(args: crate::args::Args) -> anyhow::Result<ExitCode> {
    let poke = Pokemons::load_json()?;

    let pokemon = poke
        .get_all()
        .iter()
        .find(|p| p.name.get("en").unwrap().to_lowercase() == args.pokemon_name.to_lowercase())
        .ok_or_else(|| anyhow::anyhow!("Invalid Pokemon name: {}", args.pokemon_name))?;

    let art_path = pokemon.get_sprite_path(&args.form, args.shiny)?;

    let pokemon_sprite = load_pokemon_art(&art_path)?;

    let art = std::str::from_utf8(&pokemon_sprite)?;

    // Print the pokemon name and art.
    if !args.no_title {
        if !args.shiny {
            println!("{}", pokemon.name.get("en").unwrap());
        } else {
            println!("{} (shiny)", pokemon.name.get("en").unwrap());
        }
    }
    print!("{}", art);

    let exit_code = ExitCode::from(0);
    Ok(exit_code)
}

/// Top level entry point for printing a random pokemon to the terminal
fn print_random_pokemon(args: crate::args::Args) -> anyhow::Result<ExitCode> {
    use rand::rng;

    let poke = Pokemons::load_json()?;

    let rand = rng().random_range(0..poke.len());
    let pokemon = poke.get_all().get(rand).unwrap();

    static SHINY_PROBABILITY: f64 = 1.0 / 50.00;
    static FORM_PROBABILITY: f64 = 1.0 / 10.00;

    let show_shiny = args.shiny || rng().random_bool(SHINY_PROBABILITY);

    // if show_shiny is true, set show_form to false.
    let show_form: bool = if !show_shiny {
        rng().random_bool(FORM_PROBABILITY)
    } else {
        false
    };

    // Get an random form for the pokemon if show_form is true AND pokemon.forms is not empty.
    let form: Option<String> = if show_form & !pokemon.forms.is_empty() {
        let total_forms = pokemon.forms.len();
        let form = pokemon
            .forms
            .get(rng().random_range(0..total_forms))
            .unwrap()
            .clone();
        Some(form)
    } else {
        None
    };

    let art_path = pokemon.get_sprite_path(&form, show_shiny)?;

    let pokemon_sprite = load_pokemon_art(&art_path)?;

    let art = std::str::from_utf8(&pokemon_sprite)?;

    // print the pokemon name.
    if !args.no_title {
        if !show_shiny {
            println!("{}", pokemon.name.get("en").unwrap());
        } else {
            println!("{} (shiny)", pokemon.name.get("en").unwrap());
        }

        // print the pokemon form if set.
        if let Some(form) = &form {
            println!("Form: {}", form);
        }
    }
    print!("{}", art);

    let exit_code = ExitCode::from(0);
    Ok(exit_code)
}

/// Top level entry point for printing random pokemon from the list of given pokemons to terminal
fn print_random_pokemon_by_name(args: crate::args::Args) -> anyhow::Result<ExitCode> {
    use pokemon::Pokemon;
    use rand::rng;
    use rand::seq::IteratorRandom;

    let pokes = args.pokemon_names_for_random;

    // load pokemon.json
    let poke = Pokemons::load_json()?;

    // check every pokemon name in pokemon.json to rule out any invalid pokemon name.
    //
    // update the rand_pokemon list with the Pokemon that matches the name
    let mut rand_pokemons = Vec::<&Pokemon>::new();
    {
        for rand_pokemon in pokes.iter() {
            let mut found = false;

            for pokemon in poke.get_all().iter() {
                if pokemon.name.get("en").unwrap().to_ascii_lowercase()
                    == rand_pokemon.to_ascii_lowercase()
                {
                    rand_pokemons.push(pokemon);
                    found = true;
                    break;
                }
            }

            if !found {
                return Err(anyhow::anyhow!("invalid Pokemon Name: {rand_pokemon}"));
            }
        }
    }

    let pokemon = rand_pokemons.iter().choose(&mut rand::rng()).unwrap();

    static SHINY_PROBABILITY: f64 = 1.0 / 50.00;
    static FORM_PROBABILITY: f64 = 1.0 / 10.00;

    let show_shiny = args.shiny || rng().random_bool(SHINY_PROBABILITY);

    // if show_shiny is true, set show_form to false.
    let show_form: bool = if !show_shiny {
        rng().random_bool(FORM_PROBABILITY)
    } else {
        false
    };

    // Get an random form for the pokemon if show_form is true AND pokemon.forms is not empty.
    let form: Option<String> = if show_form & !pokemon.forms.is_empty() {
        let total_forms = pokemon.forms.len();
        let form = pokemon
            .forms
            .get(rng().random_range(0..total_forms))
            .unwrap()
            .clone();
        Some(form)
    } else {
        None
    };

    let art_path = pokemon.get_sprite_path(&form, show_shiny)?;
    let pokemon_sprite = load_pokemon_art(&art_path)?;
    let art = std::str::from_utf8(&pokemon_sprite)?;

    // print the pokemon name.
    if !args.no_title {
        if !show_shiny {
            println!("{}", pokemon.name.get("en").unwrap());
        } else {
            println!("{} (shiny)", pokemon.name.get("en").unwrap());
        }

        // print the pokemon form if set.
        if let Some(form) = &form {
            println!("Form: {}", form);
        }
    }
    print!("{}", art);

    let exit_code = ExitCode::from(0);
    Ok(exit_code)
}

fn print_random_pokemon_by_gen(args: crate::args::Args) -> anyhow::Result<ExitCode> {
    use rand::seq::IteratorRandom;

    // load pokemon.json
    let poke = Pokemons::load_json()?;

    let pokemon = poke
        .get_all()
        .iter()
        .filter(|&p| args.gen_value.contains(&p.r#gen))
        .choose(&mut rand::rng())
        .unwrap();

    static SHINY_PROBABILITY: f64 = 1.0 / 50.00;

    let show_shiny = args.shiny || rand::rng().random_bool(SHINY_PROBABILITY);

    let art_path = pokemon.get_sprite_path(&None, show_shiny)?; // cannot have any
    // value for Form
    let pokemon_sprite = load_pokemon_art(&art_path)?;
    let art = std::str::from_utf8(&pokemon_sprite)?;

    // print the pokemon name.
    if !args.no_title {
        if !show_shiny {
            println!("{}", pokemon.name.get("en").unwrap());
        } else {
            println!("{} (shiny)", pokemon.name.get("en").unwrap());
        }
    }
    print!("{}", art);

    let exit_code = ExitCode::from(0);
    Ok(exit_code)
}
