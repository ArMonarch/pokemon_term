use serde::{Deserialize, Serialize};

/// Struct that represent an single pokemon entity.
///
/// Represents their name , desc, index in pokedex, generation, availabel forms.
/// This is parsed from json file that contains all the pokemon available
#[derive(Debug, Deserialize, Serialize)]
pub struct Pokemon {
    idx: u32,
    slug: String,
    r#gen: u8,
    pub name: std::collections::HashMap<String, String>,
    desc: std::collections::HashMap<String, String>,
    forms: Vec<String>,
}
