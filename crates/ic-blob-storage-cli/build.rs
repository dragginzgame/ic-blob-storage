//! Capture descriptive CLI input provenance; this does not attest the compiled executable.
use ic_build::artifacts::{InputEvidence, InputEvidenceSpec};
use std::{env, path::PathBuf};

fn main() {
    let package =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Cargo package directory"))
            .canonicalize()
            .expect("physical package directory");
    let workspace = package
        .parent()
        .and_then(std::path::Path::parent)
        .expect("virtual workspace root");
    let inputs = [
        package.join("src"),
        package.join("build.rs"),
        package.join("Cargo.toml"),
        workspace.join("Cargo.toml"),
        workspace.join("Cargo.lock"),
    ];
    let mut spec = InputEvidenceSpec::new(b"ic-blob-storage/cli-source".to_vec());
    for input in inputs {
        assert!(input.exists(), "required source input {}", input.display());
        let path = input
            .to_str()
            .expect("Cargo directive requires UTF-8 input path");
        assert!(
            !path.contains(['\n', '\r']),
            "Cargo directive requires a single-line path"
        );
        println!("cargo:rerun-if-changed={path}");
        spec = spec.with_input(input);
    }
    let evidence = InputEvidence::capture(&spec).expect("capture CLI source inventory");
    let digest = evidence
        .digest_relative_to(workspace)
        .expect("relocatable CLI input digest");
    println!("cargo:rustc-env=BLOB_CLI_SOURCE_SHA256={}", digest.to_hex());
}
