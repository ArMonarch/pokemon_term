macro_rules! write {
    ($($tt:tt)*) => { std::write!($($tt)*).unwrap();}
}

/// Generate a short Version strign in the form `pokemon-term x.y.z`
pub fn version_short() -> String {
    let app_name = "pokemon-term";
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("N/A");
    format!("{app_name} {version}")
}

/// Generate a longer multi-line version string.
///
/// This includes not only the version of pokemon-term but some other information about its build.
/// For example, Git rev hash
pub fn version_long() -> String {
    use std::fmt::Write;

    let mut output = String::new();

    let app_name = "pokemon-term";
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("N/A");
    let authors = option_env!("CARGO_PKG_AUTHORS").unwrap_or("N/A");
    let git_hash = option_env!("POKEMON_TERM_BUILD_GIT_HASH").unwrap_or("");

    write!(output, "{app_name} {version} (rev {git_hash})");
    write!(output, "\n");
    write!(output, "{authors}");

    output
}
