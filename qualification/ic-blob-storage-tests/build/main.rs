//! Prepare Wire metadata only when the callable API qualifier is selected.

#[cfg(feature = "api-manifest")]
mod contracts;

fn main() {
    println!("cargo:rerun-if-changed=build");
    #[cfg(feature = "api-manifest")]
    contracts::generate();
}
