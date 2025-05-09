#[derive(Debug)]
pub struct Args {
    // Essential Arguments.
    pub special: Option<SpecialMode>,
    pub mode: Mode,
    pub positional: Vec<String>,
    // Everything Else, Sorted lexicographically.
    pub pokemon_name: String,
    pub form: Option<String>,
    pub shiny: bool,
    pub list_with_forms: bool,
    pub pokemon_names_for_random: Vec<String>,
    pub gen_value: Vec<u8>,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            special: None,
            mode: Mode::Regular,
            positional: Vec::new(),
            pokemon_name: String::new(),
            form: None,
            shiny: false,
            list_with_forms: false,
            pokemon_names_for_random: Vec::new(),
            gen_value: Vec::new(),
        }
    }
}

/// The overall mode that pokemon-term should operate in.
///
/// The point of putting these in one enum is that they are all mutually
/// exclusive and override one another.
///
/// Note that -h/--help and -V/--version are not included in this because
/// they always overrides everything else, regardless of where it appears
/// in the command line. They are treated as "special" modes that short-circuit
/// ripgrep's usual flow.
#[derive(Debug)]
pub enum Mode {
    List,
    Regular,
    Random,
    RandomByNames,
    RandomByGen,
}

impl Mode {
    /// Update this mode to the new mode while implementing various overrides semantics. For
    /// example, Regular mode cannot replace non-Reguler mode.
    pub fn update(&mut self, new: Mode) {
        match self {
            // If we are in regular mode any mode can override it.
            Mode::Regular => {
                *self = new;
            }
            _ => {
                // Once we are in non-Reguler mode, other non-Reguler mode can override it. But
                // Regular mode cannot.
                eprintln!("found argument `` which wasn't expected, or isn't valid for this contex")
            }
        }
    }
}

/// A "special" mode that supercedes everything else.
///
/// When one of these mode is present, it overrides everything else and causes pokemon-term to
/// short-circuit. In particular, we avoid parsing anymore arguments which can fail for various
/// reasons
#[derive(Debug)]
pub enum SpecialMode {
    HelpShort,
    HelpLong,
    VersionShort,
    VersionLong,
}
