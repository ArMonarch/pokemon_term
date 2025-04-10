use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

/// Struct that represent an single pokemon entity.
///
/// Represents their name , desc, index in pokedex, generation, availabel forms.
/// This is parsed from json file that contains all the pokemon available
#[derive(Debug, Deserialize, Serialize)]
pub struct Pokemon {
    idx: u32,
    pub slug: String,
    r#gen: u8,
    pub name: std::collections::HashMap<String, String>,
    desc: std::collections::HashMap<String, String>,
    forms: Vec<String>,
}

/// Represents an Vec of `Pokemons` entity.
#[derive(Debug)]
pub struct Pokemons(Vec<Pokemon>);

impl<'a> Pokemons {
    /// Load `Vec<Pokemon>` from json file.
    ///
    /// The json file path must be assets/pokemon.json relative to the binary.
    pub fn load_json() -> anyhow::Result<Pokemons> {
        const POKEMON_JSON_PATH: &str = "assets/pokemon.json";

        let mut buffer = String::new();
        File::open(POKEMON_JSON_PATH)
            .with_context(|| format!("assets/pokemon.json not found.\nmake sure assets directory is present along side the binary.\n"))?.read_to_string(&mut buffer)?;

        let p: Vec<Pokemon> = serde_json::from_str(&buffer)?;

        Ok(Pokemons(p))
    }

    /// Returns a slice over all the `Pokemon` in the `Pokemons`.
    pub fn pokemons(&'a self) -> &'a [Pokemon] {
        &self.0
    }
}
