//! The main entry point for pokemon-term

use std::process::ExitCode;

mod args;
mod flags;
mod parse;

fn main() -> ExitCode {
    let args = flags::parse();

    match run(args) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {:#}", err);
            return ExitCode::from(1);
        }
    }
}

/// The main entry point for pokemon-term.
fn run(args: parse::ParseResult<args::Args>) -> anyhow::Result<ExitCode> {
    use args::Mode;
    use parse::ParseResult;

    let args = match args {
        ParseResult::Ok(args) => args,
        ParseResult::Err(err) => return Err(err),
        ParseResult::Special(mode) => return special(mode),
    };

    match args.mode {
        Mode::Regular => {}
        Mode::Random => {}
    }

    anyhow::bail!("Not Implemented")
}

fn special(_mode: args::SpecialMode) -> anyhow::Result<ExitCode> {
    unimplemented!()
}
