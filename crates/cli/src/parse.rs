use anyhow::Context;
use std::ffi::OsString;

use crate::{args, flags::FlagLookup};

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
    info: Vec<crate::flags::FlagInfo>,
}

impl Parser {
    /// Create a new Parser.
    ///
    /// This always creates the same Parser and only does it once. Callers may call this
    /// repeatedly, and the parser will only be build once.
    pub fn new() -> &'static Parser {
        use std::sync::OnceLock;

        use crate::flags::FLAGS;
        use crate::flags::FlagMap;
        use crate::flags::{FlagInfo, FlagInfoKind};

        /// Since a parser's state is immutable and completely determined by FLAGS, and since FLAGS
        /// is a constant, we can initialize it exactly once.
        static P: OnceLock<Parser> = OnceLock::new();

        P.get_or_init(|| {
            let mut info = Vec::new();

            for &flag in FLAGS.iter() {
                // Insert the flag given long name.

                info.push(FlagInfo {
                    flag,
                    name: Ok(flag.name_long()),
                    kind: FlagInfoKind::Standard,
                });

                // Insert the flag short name is not None.
                if let Some(byte) = flag.name_short() {
                    info.push(FlagInfo {
                        flag,
                        name: Err(byte),
                        kind: FlagInfoKind::Standard,
                    });
                }

                // Insert the flag negated name if not None.
                if let Some(name) = flag.name_negated() {
                    info.push(FlagInfo {
                        flag,
                        name: Ok(name),
                        kind: FlagInfoKind::Negated,
                    })
                }
            }

            let map = FlagMap::new(&info);
            Parser { map, info }
        })
    }

    pub fn parse<I, O>(&self, rawargs: I, args: &mut crate::args::Args) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = O>,
        O: Into<OsString>,
    {
        use crate::flags::FlagInfoKind;
        use crate::flags::FlagLookup;
        use crate::flags::FlagValue;

        let mut p = lexopt::Parser::from_args(rawargs);

        while let Some(arg) = p.next().context("invalid CLI arguments")? {
            let lookup = match arg {
                lexopt::Arg::Value(val) => {
                    match val.into_string() {
                        Ok(str) => args.positional.push(str),
                        Err(os_str) => {
                            anyhow::bail!("failed to convert OsString: {:?} to String", os_str)
                        }
                    };
                    continue;
                }

                lexopt::Arg::Short(x) if x == 'h' => {
                    // Special case -h | --help, since behavior is different based on wheather
                    // short or long flag is given.
                    args.special = Some(args::SpecialMode::HelpShort);
                    continue;
                }

                lexopt::Arg::Short(x) if x == 'v' => {
                    // Special case -v | --version, since behavior is different based on wheather
                    // short or long flag is given.
                    args.special = Some(args::SpecialMode::VersionShort);
                    continue;
                }

                lexopt::Arg::Short(x) => self.find_short(x),

                lexopt::Arg::Long(str) if str == "help" => {
                    // Special case -h | --help, since behavior is different based on wheather
                    // short or long flag is given.
                    args.special = Some(args::SpecialMode::HelpLong);
                    continue;
                }

                lexopt::Arg::Long(str) if str == "version" => {
                    // Special case -v | --version, since behavior is different based on wheather
                    // short or long flag is given.
                    args.special = Some(args::SpecialMode::VersionLong);
                    continue;
                }

                lexopt::Arg::Long(str) => self.find_long(str),
            };

            let mat = match lookup {
                FlagLookup::UnrecognizedShort(ch) => anyhow::bail!("unrecognized flag -{ch}"),
                FlagLookup::UnrecognizedLong(str) => anyhow::bail!("unrecognized flag --{str}"),
                FlagLookup::Match(mat) => mat,
            };

            let val = if mat.kind == FlagInfoKind::Negated {
                FlagValue::<OsString, bool>::Switch(false)
            } else if mat.flag.is_switch() {
                FlagValue::<OsString, bool>::Switch(true)
            } else {
                FlagValue::<OsString, bool>::Value(
                    p.value()
                        .with_context(|| format!("missing value for flag {:?}", mat))?,
                )
            };

            mat.flag
                .update(val, args)
                .with_context(|| format!("error parsing flag {:?}", mat))?;
        }

        Ok(())
    }

    /// Look for a flag by its short name.
    fn find_short(&self, ch: char) -> crate::flags::FlagLookup<'_> {
        use crate::flags::FlagLookup;

        if !ch.is_ascii() {
            return FlagLookup::UnrecognizedShort(ch);
        }

        let byte = u8::try_from(ch).unwrap();
        let Some(index) = self.map.find(&[byte]) else {
            return FlagLookup::UnrecognizedShort(ch);
        };

        FlagLookup::Match(&self.info[index])
    }

    /// Look for a flag by its long name.
    fn find_long(&self, str: &str) -> crate::flags::FlagLookup<'_> {
        let Some(index) = self.map.find(str.as_bytes()) else {
            return FlagLookup::UnrecognizedLong(str.to_string());
        };
        FlagLookup::Match(&self.info[index])
    }
}
