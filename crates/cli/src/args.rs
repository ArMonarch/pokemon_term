#[derive(Debug)]
pub struct Args {
    // Essential Arguments.
    pub special: SpecialMode,
    pub mode: Mode,
    pub positional: Vec<String>,
    // Everything Else, Sorted lexicographically.
    pub pokemon_name: String,
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
