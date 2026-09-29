//! Explicit public-source capture; no account, gateway or paid-provider methods.
#[cfg(not(target_family = "wasm"))]
mod probe;
#[cfg(not(target_family = "wasm"))]
fn main() -> std::process::ExitCode {
    probe::run()
}
#[cfg(target_family = "wasm")]
fn main() {}
