//! Contributor-only mixed-version package qualification; never a publication command.
#[cfg(not(target_family = "wasm"))]
#[derive(clap::Parser)]
#[command(
    name = "blob-package-verify",
    about = "Qualify independently versioned package archives without publication"
)]
struct Arguments {
    #[arg(value_name = "workspace")]
    workspace: std::path::PathBuf,
    #[arg(value_name = "archives")]
    archives: std::path::PathBuf,
    #[arg(value_name = "evidence")]
    evidence: std::path::PathBuf,
    #[arg(value_name = "contracts-version")]
    contracts_version: String,
    #[arg(value_name = "client-version")]
    client_version: String,
    #[arg(value_name = "engine-version")]
    engine_version: String,
}

fn main() {
    #[cfg(not(target_family = "wasm"))]
    {
        use clap::Parser;
        use ic_build::cargo::packages::{
            PackageArchiveSpec, PackageCommandError, PackageQualificationLimits,
            qualify_package_archives,
        };
        use ic_build::cargo::selection::CargoFeatureSelection;
        use ic_host_process::tool::{OutputLimit, OutputLimits, ToolError};
        use std::process::Command;
        let args = Arguments::parse();
        let packages = [
            ("ic-blob-storage-contracts", &args.contracts_version, true),
            ("ic-blob-storage-client", &args.client_version, false),
            ("ic-blob-storage", &args.engine_version, false),
        ]
        .map(|(name, version, patch_registry)| PackageArchiveSpec {
            name,
            version,
            patch_registry,
        });
        let result = qualify_package_archives(
            &args.workspace,
            &args.archives,
            &args.evidence,
            &packages,
            &CargoFeatureSelection::default(),
            PackageQualificationLimits {
                process: OutputLimits {
                    stdout: OutputLimit::Terminate(64 * 1024 * 1024),
                    stderr: OutputLimit::Terminate(64 * 1024 * 1024),
                    timeout: None,
                },
                file_bytes: 64 * 1024 * 1024,
                source_entries: 100_000,
                source_depth: 64,
            },
            || Command::new("cargo"),
            |archive, destination| {
                let mut command = Command::new("tar");
                command.arg("-xzf").arg(archive).arg("-C").arg(destination);
                command
            },
        );
        if let Err(error) = result {
            eprintln!("{error}; evidence retained: {}", error.evidence.display());
            let status = match error
                .cause
                .downcast_ref::<PackageCommandError>()
                .and_then(|error| error.execution.as_ref())
            {
                Some(ToolError::Execution(error)) => error
                    .evidence
                    .status
                    .and_then(|status| {
                        if let Some(code) = status.code() {
                            return Some(code);
                        }
                        #[cfg(unix)]
                        {
                            use std::os::unix::process::ExitStatusExt as _;
                            status.signal().map(|signal| 128 + signal)
                        }
                        #[cfg(not(unix))]
                        {
                            None
                        }
                    })
                    .filter(|code| *code > 0)
                    .unwrap_or(1),
                _ => 1,
            };
            std::process::exit(status);
        }
        println!("Package archive graph, tests and final integrity checks passed.");
    }
}
