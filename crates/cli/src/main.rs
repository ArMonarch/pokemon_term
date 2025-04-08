//! The main entry point for pokemon-term

use core::str;
use std::{io::Read, process::ExitCode};

mod args;
mod flags;
mod parse;

mod util;

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

use serde::{Deserialize, Serialize};

/// Struct that represent an single pokemon entity.
///
/// Represents their name , desc, index in pokedex, generation, availabel forms.
/// This is parsed from json file that contains all the pokemon available
#[derive(Debug, Deserialize, Serialize)]
struct Pokemon<'a> {
    idx: u32,
    slug: &'a str,
    r#gen: u8,
    name: std::collections::HashMap<String, String>,
    desc: std::collections::HashMap<String, String>,
    forms: Vec<&'a str>,
}

/// Top level entry point for listing all pokemons
///
/// This function parse the assets/pokemons.json to get the list of available pokemons available and prints
/// the list to the terminal.
fn list_pokemons(_args: crate::args::Args) -> anyhow::Result<ExitCode> {
    use anyhow::Context;
    use std::fs::File;

    use util::write;

    let exit_code = ExitCode::from(0);

    let json_path = "assets/pokemon.json";

    let mut result = String::new();
    let _pokemon_json = File::open(json_path)
        .with_context(|| format!("assets/pokemon.json not found.\nmake sure assets directory is present along side the binary.\n"))?
        .read_to_string(&mut result)?;

    let json_data: Vec<Pokemon> = serde_json::from_str(&result)?;

    let mut list_output = String::new();

    // TODO: format the list of pokemon in 3 columns for better readibility rather than as a single
    // column
    for (i, pokemon) in json_data.iter().enumerate() {
        if i > 0 {
            write(&mut list_output, "\n");
        }
        write(&mut list_output, pokemon.name.get("en").unwrap());
    }

    println!("{}", list_output);

    Ok(exit_code)
}

/// Top level entry point for printing pokemon to the terminal
fn print_pokemon(_args: crate::args::Args) -> anyhow::Result<ExitCode> {
    unimplemented!("Not Implemented")
}

/// Top level entry point for printing a random pokemon to the terminal
fn print_random_pokemon(_args: crate::args::Args) -> anyhow::Result<ExitCode> {
    unimplemented!("Not Implemented")
}

/// Top level entry point for printing random pokemon from the list of given pokemons to terminal
fn print_random_pokemon_by_name(_args: crate::args::Args) -> anyhow::Result<ExitCode> {
    unimplemented!("Not Implemented")
}
