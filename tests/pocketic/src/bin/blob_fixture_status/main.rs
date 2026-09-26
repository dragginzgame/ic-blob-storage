//! Read-only diagnostics for explicitly selected local `PocketIC` fixtures.

#[cfg(not(target_family = "wasm"))]
fn main() -> std::process::ExitCode {
    // The SDK panics on transport failure; the client translates that boundary to
    // a stable JSON error. Do not emit its unbounded remote panic text on stderr.
    std::panic::set_hook(Box::new(|_| {}));
    let result = ic_blob_storage_pocketic_tests::operator::run(
        &std::env::args().skip(1).collect::<Vec<_>>(),
    );
    println!("{}", result.output);
    std::process::ExitCode::from(result.exit_code)
}

#[cfg(target_family = "wasm")]
fn main() {}
