//! Query-only prepared-inventory inspection against an explicit local fixture.

#[cfg(not(target_family = "wasm"))]
fn main() -> std::process::ExitCode {
    std::panic::set_hook(Box::new(|_| {}));
    let result = ic_blob_storage_pocketic_tests::operator::run_inventory(
        &std::env::args().skip(1).collect::<Vec<_>>(),
    );
    println!("{}", result.output);
    std::process::ExitCode::from(result.exit_code)
}

#[cfg(target_family = "wasm")]
fn main() {}
