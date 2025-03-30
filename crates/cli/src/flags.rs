use std::fmt::Debug;
use std::process::ExitCode;

/// A trait that encapsulates the definition of an optional flag for pokemon-term
///
/// Note that each implementation of this trait requires a long flag name,
/// but can also optionally have a short version and even a negation flag.
/// For example, the `-E/--encoding` flag accepts a value, but it also has a
/// `--no-encoding` negation flag for reverting back to "automatic" encoding
/// detection. All three of `-E`, `--encoding` and `--no-encoding` are provided
/// by a single implementation of this trait.

trait Flag: Debug {
    fn is_switch() -> bool;

    fn is_multivalued() -> bool;

    fn name_negated() -> Option<&'static str> {
        None
    }

    fn name_short() -> Option<u8> {
        None
    }

    fn name_long() -> &'static str;

    fn doc_short() -> &'static str;

    fn doc_long() -> &'static str;

    fn update() -> anyhow::Result<ExitCode>;
}

use crate::args;
use crate::parse::ParseResult;

pub fn parse() -> ParseResult<args::Args> {
    unimplemented!();
}
