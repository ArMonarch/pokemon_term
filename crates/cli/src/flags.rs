use std::ffi::OsString;
use std::fmt::Debug;
use std::process::ExitCode;

/// Represents a value parsed from the command line.
///
/// This doesn't include the corrsponding flag, but value come in one of three form:
/// 1. A switch (on or off),
/// 2. an arbitary value,
/// 3. Vec of arbitary value,
enum FlagValue<T> {
    /// A flag that is either on or off.
    Switch(bool),
    /// A flag that comes with an arbitrary user value.
    Value(T),
    /// A flag that comes with an vec of arbitrary user value.
    MultiValued(Vec<T>),
}

impl<T> FlagValue<T> {
    /// Returns the yes or no value of the switch.
    ///
    /// If this flag value is not switch, then this panics.
    ///
    /// This is useful when writing the implementation of `Flag::update`.
    /// namely, caller usually know whether a switch, val, vec is expected.
    /// If the flag is something different, then it indicates a bug, and thus a panic is
    /// acceptable.
    fn unwrap_switch(self) -> bool {
        match self {
            FlagValue::Switch(bool) => bool,
            FlagValue::Value(_) => unreachable!("got flag value but expected switch"),
            FlagValue::MultiValued(_) => unreachable!("got vec of flag value but expected switch"),
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
    fn unwrap_value(self) -> T {
        match self {
            FlagValue::Value(val) => val,
            FlagValue::Switch(_) => unreachable!("got switch but expected flag value"),
            FlagValue::MultiValued(_) => {
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
    fn unwrap_vec(self) -> Vec<T> {
        match self {
            FlagValue::MultiValued(vec) => vec,
            FlagValue::Switch(_) => unreachable!("got switch but expected vec of flag value"),
            FlagValue::Value(_) => unreachable!("get flag of value but expected flag value"),
        }
    }
}

/// The kind of flag that is beign matched
#[derive(Debug)]
pub(crate) enum FlagInfoKind {
    /// A standard flag, e.g., --name
    Standard,

    /// A negation of a standard flag, e.g., ...
    Negated,

    /// A alias for standard flag, e.g.,
    Alias,
}

/// The flag name that represents a flag in `FLAGS`.
#[derive(Debug)]
pub(crate) enum FlagName {
    Char(char),
    String(&'static str),
}

#[derive(Debug)]
pub(crate) struct FlagMap {
    map: std::collections::HashMap<u8, usize>,
}

#[derive(Debug)]
pub(crate) struct FlagInfo {
    flag: &'static dyn Flag,
    name: FlagName,
    kind: FlagInfoKind,
}

/// A list of all flags in pokemon-term via implementations of `Flag`.
///
/// The order of these flags matter. It determines the order of the flags in
/// the generated documentation (`-h`, `--help` and the man page) within each
/// category. (This is why the deprecated flags are last.)
const FLAGS: &[&dyn Flag] = &[];

/// A trait that encapsulates the definition of an optional flag for pokemon-term
///
/// Note that each implementation of this trait requires a long flag name,
/// but can also optionally have a short version and even a negation flag.
/// For example, the `-E/--encoding` flag accepts a value, but it also has a
/// `--no-encoding` negation flag for reverting back to "automatic" encoding
/// detection. All three of `-E`, `--encoding` and `--no-encoding` are provided
/// by a single implementation of this trait.
trait Flag: Debug + Send + Sync + 'static {
    /// Returns true if this flag is a switch. When a flag is a switch, the
    /// CLI parser will not look for a value after the flag is seen.
    fn is_switch(&self) -> bool;

    /// Returns true if this flag is multivalued. When a flag is a multivalued, the
    /// CLI parser will look for multiple value after the flag is seen.
    fn is_multivalued(&self) -> bool;

    fn name_negated(&self) -> Option<&'static str> {
        None
    }

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

    /// Returns the variable name describing the type of value this flag accepts. This should
    /// always be set for non-switch flags and never set for switch flags.
    ///
    /// For example, the `--max-count` flag has its variable name set to `NUM`.
    ///
    /// The convention is to capitalize variable names.
    ///
    /// By default this return None.
    fn doc_variable(&self) -> Option<&'static str>;

    /// A (very) short documentation string describing what this flag does.
    ///
    /// This may sacrifice "Proper English" in order to be as terse as possible. Generally ensure
    /// that `pokemon-term -h` doesn't have lines that exceed 99 columns.
    fn doc_short(&self) -> &'static str;

    /// A (possibly very) longer documentation string describing in full detail what this flag
    /// does. This should be in mandoc/mdoc format.
    fn doc_long(&self) -> &'static str;

    /// Given the parsed value (which might just be a switch), this should update the state in
    /// `args` based on the value given for this flag.
    ///
    /// This may update state for other flags as appropriate.
    ///
    /// The `-V | --version` and `-h | --help` flags are treated as special in the parser and
    /// should nothing here.
    fn update(
        &self,
        val: FlagValue<OsString>,
        args: &mut crate::args::Args,
    ) -> anyhow::Result<ExitCode>;
}

/// -n | --name
#[derive(Debug)]
struct Name;

/// -l | --list
#[derive(Debug)]
struct List;

/// -r | --random
#[derive(Debug)]
struct Random;

/// -rn | --random-by-names
#[derive(Debug)]
struct RandomByNames;

/// -s | --shiny
#[derive(Debug)]
struct Shiny;

use crate::args::Args;
use crate::parse::ParseResult;

pub fn parse() -> ParseResult<Args> {
    let parser = crate::parse::Parser::new();

    let err = anyhow::anyhow!("Not Implemented");
    ParseResult::Err(err)
}
