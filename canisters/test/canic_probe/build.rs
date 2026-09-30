//! Compile the maintained Canic role and bind an explicit local fixture identity.
fn main() {
    canic::build!("canic.toml");
    // A fixed identity is fixture authority only, not a finalized production artifact.
    println!("cargo:rustc-env=CANIC_RELEASE_BUILD_ID={}", "01".repeat(32));
}
