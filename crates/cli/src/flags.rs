use std::ffi::OsString;
use std::fmt::Debug;

/// Represents a value parsed from the command line.
///
/// This doesn't include the corrsponding flag, but value come in one of three form:
/// 1. A switch (on or off),
/// 2. an arbitary value,
/// 3. Vec of arbitary value,
#[derive(Debug)]
pub enum FlagValue<I, O> {
    /// A flag that is either on or off.
    Switch(O),
    /// A flag that comes with an arbitrary user value.
    Value(I),
    /// A flag that comes with an vec of arbitrary user value.
    _MultiValued(Vec<I>),
}

impl<I, O> FlagValue<I, O> {
    /// Returns the yes or no value of the switch.
    ///
    /// If this flag value is not switch, then this panics.
    ///
    /// This is useful when writing the implementation of `Flag::update`.
    /// namely, caller usually know whether a switch, val, vec is expected.
    /// If the flag is something different, then it indicates a bug, and thus a panic is
    /// acceptable.
    fn unwrap_switch(self) -> O {
        match self {
            FlagValue::Switch(bool) => bool,
            FlagValue::Value(_) => unreachable!("got flag value but expected switch"),
            FlagValue::_MultiValued(_) => unreachable!("got vec of flag value but expected switch"),
        }
    }

    /// Returns the user provided value of this flag.
    ///
    /// If this flag value is either switch or vec then it panics.
    ///
    /// This is useful when writing the implementation of `Flag::update`.
    /// namely, caller usually know whether a switch, val, vec is expected.
    /// If the flag is something different, then it indicates a bug, and thus a panic is
    /// acceptable.
    fn unwrap_value(self) -> I {
        match self {
            FlagValue::Value(val) => val,
            FlagValue::Switch(_) => unreachable!("got switch but expected flag value"),
            FlagValue::_MultiValued(_) => {
                unreachable!("got vec of flag values but expected an flag value")
            }
        }
    }

    /// Returns the vec of user provided values for this flag.
    ///
    /// if this flag is not vec of values, then it panics.
    ///
    /// This is useful when writing the implementation of `Flag::update`.
    /// namely, caller usually know whether a switch, val, vec is expected.
    /// If the flag is something different, then it indicates a bug, and thus a panic is
    /// acceptable.
    fn _unwrap_vec(self) -> Vec<I> {
        match self {
            FlagValue::_MultiValued(vec) => vec,
            FlagValue::Switch(_) => unreachable!("got switch but expected vec of flag value"),
            FlagValue::Value(_) => unreachable!("get flag of value but expected flag value"),
        }
    }
}

/// The kind of flag that is beign matched
#[derive(Debug, PartialEq)]
pub(crate) enum FlagInfoKind {
    /// A standard flag, e.g., --name
    Standard,

    /// A negation of a standard flag, e.g., ...
    Negated,

    /// A alias for standard flag, e.g.,
    _Alias,
}

/// The info about a flag associated with a flag's ID in the flag map.
#[derive(Debug)]
pub(crate) struct FlagInfo {
    /// The flag object and its associated metadata.
    pub flag: &'static dyn Flag,
    /// The actual name stored in the `FlagMap`.
    pub name: Result<&'static str, u8>,
    /// The type of flag that is stored for the corrsponding pattern.
    pub kind: FlagInfoKind,
}

/// A map from flag name (short, long, negated and aliases) to their ID.
///
/// Once an ID is known, it can be used to look up a flag's metadata in the parser's internal
/// state.
#[derive(Debug)]
pub(crate) struct FlagMap {
    map: std::collections::HashMap<Vec<u8>, usize>,
}

impl FlagMap {
    /// Create a new map of flags for the given flag information.
    ///
    /// The index of each flag info corresponds to its ID.
    pub fn new(infos: &[FlagInfo]) -> FlagMap {
        let mut map = std::collections::HashMap::new();

        for (index, flag) in infos.iter().enumerate() {
            match flag.name {
                Ok(str) => {
                    assert_eq!(None, map.insert(str.as_bytes().to_vec(), index))
                }
                Err(byte) => {
                    assert_eq!(None, map.insert(vec![byte], index))
                }
            }
        }

        FlagMap { map }
    }

    /// look for a match of `name` in the `Flagmap`.
    ///
    /// This only returns a match if the one found has a length equivalent to the length of the
    /// name given.
    pub fn find(&self, key: &[u8]) -> Option<usize> {
        self.map.get(key).copied()
    }
}

///  The result of looking up a flag name
#[derive(Debug)]
pub enum FlagLookup<'a> {
    Match(&'a FlagInfo),
    /// The given short name is unrecognized.
    UnrecognizedShort(char),
    /// The given long name is unrecognized.
    UnrecognizedLong(String),
}

/// A list of all flags in pokemon-term via implementations of `Flag`.
///
/// The order of these flags matter. It determines the order of the flags in
/// the generated documentation (`-h`, `--help` and the man page).
///(This is why the deprecated flags are last.)
pub(crate) const FLAGS: &[&dyn Flag] = &[
    &Name,
    &List,
    &ShowForms,
    &Shiny,
    &Form,
    &Random,
    &RandomByNames,
];

/// A trait that encapsulates the definition of an optional flag for pokemon-term
///
/// Note that each implementation of this trait requires a long flag name,
/// but can also optionally have a short version and even a negation flag.
/// For example, the `-E/--encoding` flag accepts a value, but it also has a
/// `--no-encoding` negation flag for reverting back to "automatic" encoding
/// detection. All three of `-E`, `--encoding` and `--no-encoding` are provided
/// by a single implementation of this trait.
pub(crate) trait Flag: Debug + Send + Sync + 'static {
    /// Returns true if this flag is a switch. When a flag is a switch, the
    /// CLI parser will not look for a value after the flag is seen.
    fn is_switch(&self) -> bool;

    /// Returns true if this flag is multivalued. When a flag is a multivalued, the
    /// CLI parser will look for multiple value after the flag is seen.
    fn _is_multivalued(&self) -> bool;

    /// A short single byte name for this flag. This return `None` by defult, which signifies that
    /// the flag has no short name.
    ///
    /// A byte returned must be an ASCII codepoint that is a `.` or is alpha-numeric.
    fn name_short(&self) -> Option<u8> {
        None
    }

    /// Returns the long name fo this flag. All Flag must have an "long" name.
    ///
    /// the long name must be at least 2 bytes and all of its bytes must be ASCII characters.
    fn name_long(&self) -> &'static str;

    /// ...
    fn name_negated(&self) -> Option<&'static str> {
        None
    }

    /// Returns the variable name describing the type of value this flag accepts. This should
    /// always be set for non-switch flags and never set for switch flags.
    ///
    /// For example, the `--max-count` flag has its variable name set to `NUM`.
    ///
    /// The convention is to capitalize variable names.
    ///
    /// By default this return None.
    fn _doc_variable(&self) -> Option<&'static str>;

    /// A (very) short documentation string describing what this flag does.
    ///
    /// This may sacrifice "Proper English" in order to be as terse as possible. Generally ensure
    /// that `pokemon-term -h` doesn't have lines that exceed 99 columns.
    fn _doc_short(&self) -> &'static str;

    /// A (possibly very) longer documentation string describing in full detail what this flag
    /// does. This should be in mandoc/mdoc format.
    fn _doc_long(&self) -> &'static str;

    /// Given the parsed value (which might just be a switch), this should update the state in
    /// `args` based on the value given for this flag.
    ///
    /// This may update state for other flags as appropriate.
    ///
    /// The `-V | --version` and `-h | --help` flags are treated as special in the parser and
    /// should nothing here.
    fn update(
        &self,
        val: FlagValue<OsString, bool>,
        args: &mut crate::args::Args,
    ) -> anyhow::Result<()>;
}

/// -n | --name
#[derive(Debug)]
struct Name;

impl Flag for Name {
    fn is_switch(&self) -> bool {
        false
    }

    fn _is_multivalued(&self) -> bool {
        false
    }

    fn name_short(&self) -> Option<u8> {
        Some(b'n')
    }

    fn name_long(&self) -> &'static str {
        "name"
    }

    fn name_negated(&self) -> Option<&'static str> {
        None
    }

    fn _doc_variable(&self) -> Option<&'static str> {
        Some("NAME")
    }

    fn _doc_short(&self) -> &'static str {
        "Print the Pokemon by its Name. Generally spelled like in the game."
    }

    fn _doc_long(&self) -> &'static str {
        ""
    }

    fn update(
        &self,
        val: FlagValue<OsString, bool>,
        args: &mut crate::args::Args,
    ) -> anyhow::Result<()> {
        let name = match val.unwrap_value().into_string() {
            Ok(str) => str,
            Err(os_str) => anyhow::bail!(
                "failed to parse value {:?}, for flag \"-n\" | \"--name\"",
                os_str
            ),
        };

        // update pokemon name only if its already empty.
        // else return err.
        if !args.pokemon_name.is_empty() {
            anyhow::bail!(
                "tried to overwrite flag '-n' | '--name' '{}' <- '{}'.",
                args.pokemon_name,
                name
            )
        }

        args.pokemon_name = name;

        Ok(())
    }
}

/// -s | --shiny
#[derive(Debug)]
struct Shiny;

impl Flag for Shiny {
    fn is_switch(&self) -> bool {
        true
    }

    fn _is_multivalued(&self) -> bool {
        false
    }

    fn name_short(&self) -> Option<u8> {
        Some(b's')
    }

    fn name_long(&self) -> &'static str {
        "shiny"
    }

    fn name_negated(&self) -> Option<&'static str> {
        None
    }

    fn _doc_variable(&self) -> Option<&'static str> {
        None
    }

    fn _doc_short(&self) -> &'static str {
        "Print the shiny version of the pokemon."
    }

    fn _doc_long(&self) -> &'static str {
        ""
    }

    fn update(
        &self,
        val: FlagValue<OsString, bool>,
        args: &mut crate::args::Args,
    ) -> anyhow::Result<()> {
        args.shiny = val.unwrap_switch();
        Ok(())
    }
}

/// -f | --form
#[derive(Debug)]
struct Form;

impl Flag for Form {
    fn is_switch(&self) -> bool {
        false
    }

    fn _is_multivalued(&self) -> bool {
        false
    }

    fn name_short(&self) -> Option<u8> {
        Some(b'f')
    }

    fn name_long(&self) -> &'static str {
        "form"
    }

    fn name_negated(&self) -> Option<&'static str> {
        None
    }

    fn _doc_variable(&self) -> Option<&'static str> {
        Some("FORM")
    }

    fn _doc_short(&self) -> &'static str {
        "Print the given form version of the pokemon."
    }

    fn _doc_long(&self) -> &'static str {
        ""
    }

    fn update(
        &self,
        val: FlagValue<OsString, bool>,
        args: &mut crate::args::Args,
    ) -> anyhow::Result<()> {
        let form = match val.unwrap_value().into_string() {
            Ok(str) => str,
            Err(os_str) => anyhow::bail!(
                "failed to parse value {:?}, for flag \"-f\" | \"--form\"",
                os_str
            ),
        };

        // update pokemon form only if its already empty.
        // else return err.
        if !args.form.is_none() {
            anyhow::bail!(
                "tried to overwrite flag '-f' | '--form' '{}' <- '{}'.",
                args.form.as_ref().unwrap(),
                form
            )
        }

        args.form = Some(form);

        Ok(())
    }
}

/// -l | --list
#[derive(Debug)]
struct List;

impl Flag for List {
    fn is_switch(&self) -> bool {
        true
    }

    fn _is_multivalued(&self) -> bool {
        false
    }

    fn name_short(&self) -> Option<u8> {
        Some(b'l')
    }

    fn name_long(&self) -> &'static str {
        "list"
    }

    fn name_negated(&self) -> Option<&'static str> {
        None
    }

    fn _doc_variable(&self) -> Option<&'static str> {
        None
    }

    fn _doc_short(&self) -> &'static str {
        "Print a list of all pokemons"
    }

    fn _doc_long(&self) -> &'static str {
        ""
    }

    fn update(
        &self,
        val: FlagValue<OsString, bool>,
        args: &mut crate::args::Args,
    ) -> anyhow::Result<()> {
        use crate::args::Mode;

        assert!(val.unwrap_switch());

        args.mode.update(Mode::List);
        Ok(())
    }
}

/// --show-forms
#[derive(Debug)]
struct ShowForms;

impl Flag for ShowForms {
    fn is_switch(&self) -> bool {
        true
    }

    fn _is_multivalued(&self) -> bool {
        false
    }

    fn name_short(&self) -> Option<u8> {
        None
    }

    fn name_long(&self) -> &'static str {
        "show-forms"
    }

    fn name_negated(&self) -> Option<&'static str> {
        None
    }

    fn _doc_variable(&self) -> Option<&'static str> {
        None
    }

    fn _doc_short(&self) -> &'static str {
        "Show List of Pokemons with their respective forms."
    }

    fn _doc_long(&self) -> &'static str {
        ""
    }

    fn update(
        &self,
        val: FlagValue<OsString, bool>,
        args: &mut crate::args::Args,
    ) -> anyhow::Result<()> {
        args.list_with_forms = val.unwrap_switch();

        Ok(())
    }
}

/// -r | --random
#[derive(Debug)]
struct Random;

impl Flag for Random {
    fn is_switch(&self) -> bool {
        true
    }

    fn _is_multivalued(&self) -> bool {
        false
    }

    fn name_short(&self) -> Option<u8> {
        Some(b'r')
    }

    fn name_long(&self) -> &'static str {
        "random"
    }

    fn name_negated(&self) -> Option<&'static str> {
        None
    }

    fn _doc_short(&self) -> &'static str {
        "Print a Random Pokemon in the terminal. Includes shiny version and their forms."
    }

    fn _doc_long(&self) -> &'static str {
        ""
    }

    fn _doc_variable(&self) -> Option<&'static str> {
        None
    }

    fn update(
        &self,
        val: FlagValue<OsString, bool>,
        args: &mut crate::args::Args,
    ) -> anyhow::Result<()> {
        use crate::args::Mode;

        assert!(val.unwrap_switch());

        args.mode.update(Mode::Random);
        Ok(())
    }
}

/// --random-by-names
#[derive(Debug)]
struct RandomByNames;

impl Flag for RandomByNames {
    fn is_switch(&self) -> bool {
        false
    }

    fn _is_multivalued(&self) -> bool {
        false
    }

    fn name_short(&self) -> Option<u8> {
        None
    }

    fn name_long(&self) -> &'static str {
        "random-by-name"
    }

    fn name_negated(&self) -> Option<&'static str> {
        None
    }

    fn _doc_variable(&self) -> Option<&'static str> {
        Some("[Pokemon Names]")
    }

    fn _doc_short(&self) -> &'static str {
        "Print Random Pokemon from given Pokemon names. Pokemon names must be seperated by comma(',')."
    }

    fn _doc_long(&self) -> &'static str {
        ""
    }

    fn update(
        &self,
        val: FlagValue<OsString, bool>,
        args: &mut crate::args::Args,
    ) -> anyhow::Result<()> {
        let pokemon_names = match val.unwrap_value().into_string() {
            Ok(str) => str,
            Err(os_str) => anyhow::bail!(
                "failed to parse value {:?}, for flag \"-f\" | \"--form\"",
                os_str
            ),
        };

        // pokemon names must not be empty
        assert_eq!(false, pokemon_names.is_empty());

        // update pokemon names for random pokemon.
        args.pokemon_names_for_random = pokemon_names
            .split(",")
            .map(|str| str.to_string())
            .collect();

        // lastly, update the mode to random-by-name
        args.mode.update(crate::args::Mode::RandomByNames);

        Ok(())
    }
}

use crate::args::Args;
use crate::parse::ParseResult;

pub fn parse() -> ParseResult<Args> {
    let parser = crate::parse::Parser::new();
    let mut args = crate::args::Args::default();

    if let Err(err) = parser.parse(std::env::args().skip(1), &mut args) {
        return ParseResult::Err(err);
    }

    // We can bail early, if a special mode was enabled. This is basically only for version and
    // help output which shouldn't be impacted by what is done next.
    if let Some(special_mode) = args.special {
        return ParseResult::Special(special_mode);
    }

    ParseResult::Ok(args)
}
