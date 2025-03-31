use crate::args;

/// The result of parsing CLI arguments.
///
/// This is basically a `anyhow::Result<T>`, but with one extra variant that is
/// inhabited whenever pokemon-term should execute a "special" mode. That is, when a
/// user provides the `-h/--help` or `-V/--version` flags.
///
/// This special variant exists to allow CLI parsing to short circuit as
/// quickly as is reasonable. For example, it lets CLI parsing avoid reading
/// ripgrep's configuration and converting low level arguments into a higher
/// level representation.
#[derive(Debug)]
pub(crate) enum ParseResult<T> {
    Ok(T),
    Err(anyhow::Error),
    Special(args::SpecialMode),
}

/// A parser for turning a squence of command line arguments into a more strictly typed set of
/// arguments
#[derive(Debug)]
pub(crate) struct Parser {
    /// A single map that contains all possible flag names. This includes short and long names,
    /// aliases and negations . Then maps those names to indices into `info`.
    map: crate::flags::FlagMap,

    /// A map from IDs returned by the `map` to the corresponding flag information.
    info: crate::flags::FlagInfo,
}

impl Parser {
    /// Create a new Parser.
    ///
    /// This always creates the same Parser and only does it once. Callers may call this
    /// repeatedly, and the parser will only be build once.
    pub fn new() -> Parser {
        use std::sync::OnceLock;

        /// Since a parser's state is immutable and completely determined by FLAGS, and since FLAGS
        /// is a constant, we can initialize it exactly once.
        static P: OnceLock<Parser> = OnceLock::new();

        unimplemented!()
    }

    fn parse() -> anyhow::Result<()> {
        let err = anyhow::anyhow!("Not Implemented");

        Err(err)
    }
}
