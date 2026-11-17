//! Admit the supplier-owned native startup transform before any local installation.
use clap::Parser;
use ic_host_artifacts::Length;
use std::{fs::File, path::PathBuf};

#[derive(Parser)]
struct Args {
    /// Original compiler-produced Wasm.
    input: PathBuf,
    /// New prepared artifact; an existing output is never overwritten.
    output: PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (raw, _) =
        ic_host_artifacts::read(File::open(args.input)?, Length::AtMost(32 * 1024 * 1024))?;
    let prepared = ic_storage::retained_build::prepare(&raw)
        .map_err(|e| format!("native artifact preparation: {e:?}"))?;
    ic_storage::retained_build::admit(&prepared, &raw)
        .map_err(|e| format!("native artifact admission: {e:?}"))?;
    use std::io::Write;
    let mut file = File::create_new(args.output)?;
    file.write_all(&prepared)?;
    file.sync_all()?;
    Ok(())
}
