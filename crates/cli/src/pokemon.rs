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
    pub forms: Vec<String>,
}

impl Pokemon {
    pub fn get_sprite_path(&self, form: &Option<String>, shiny: bool) -> anyhow::Result<String> {
        // when shiny is true, assert form must be None.
        if shiny && form.is_some() {
            return Err(anyhow::anyhow!(
                "form value set to Some(\"{}\") when shiny set to True.",
                form.as_deref().unwrap()
            ));
        }

        Ok(format!(
            "assets/colorscripts/{}/{}",
            if shiny { "shiny" } else { "regular" },
            self.get_form_slug(form)?
        ))
    }

    fn get_form_slug(&self, form: &Option<String>) -> anyhow::Result<String> {
        Ok(if let Some(form) = form {
            format!(
                "{}-{}",
                self.slug,
                if self.forms.contains(form) {
                    form
                } else {
                    anyhow::bail!("Invalid form for {}", self.slug)
                }
            )
        } else {
            self.slug.clone()
        })
    }
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

    /// Returns the number of pokemons in the vector, also referred to as its length.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns a slice over all the `Pokemon` in the `Pokemons`.
    pub fn get_all(&'a self) -> &'a [Pokemon] {
        &self.0
    }
}
