//! Offline upload inputs and explicitly authenticated service operations.
#[cfg(not(target_family = "wasm"))]
mod native;

#[cfg(not(target_family = "wasm"))]
fn main() -> std::process::ExitCode {
    native::run()
}

// Keep native transport and identity dependencies out of workspace Wasm checks.
#[cfg(target_family = "wasm")]
fn main() {}
