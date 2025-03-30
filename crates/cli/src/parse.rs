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
