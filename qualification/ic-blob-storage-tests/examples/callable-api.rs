//! Render the public methods of the two product release Wasms for PR comparison.

use clap::Parser;
use ic_host_artifacts::{Length, methods};
use std::{error::Error, fs::File, io::Write, path::PathBuf};

///
/// Artifacts
/// Cargo-selected release modules and the comparison runner’s output directory.
///

#[derive(Parser)]
struct Artifacts {
    standalone: PathBuf,
    application: PathBuf,
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Artifacts::parse();
    for (label, path) in [
        ("standalone", args.standalone),
        ("reference-app", args.application),
    ] {
        let file = File::open(path)?;
        let length = Length::Exactly(file.metadata()?.len());
        let (methods, _) = methods::inventory(file, length)?;
        let mut output = File::create_new(args.output.join(format!("{label}.txt")))?;
        for (name, kind) in methods {
            let mode = match kind {
                methods::Kind::Query => "query",
                methods::Kind::Update => "update",
                methods::Kind::CompositeQuery => "composite_query",
            };
            writeln!(output, "{mode} {name:?}")?;
        }
    }

    Ok(())
}
