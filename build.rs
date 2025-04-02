fn main() {
    set_git_hash();
}

/// Make the current git hash available to the build as the environment variable
/// `POEKMON_TERM_BUILD_GIT_HASH`.
fn set_git_hash() {
    use std::process::Command;

    let args = &["rev-parse", "HEAD"];
    let Ok(output) = Command::new("git").args(args).output() else {
        return;
    };

    let str = String::from_utf8_lossy(&output.stdout).to_string();
    if str.is_empty() {
        return;
    }

    println!("cargo:rustc-env=POEKMON_TERM_BUILD_GIT_HASH={}", str);
}
