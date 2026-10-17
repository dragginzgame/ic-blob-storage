//! Contributor-only coordinated package qualification; never a publication command.
fn main() {
    #[cfg(not(target_family = "wasm"))]
    {
        use ic_build::cargo::packages::{
            PackageArchiveSpec, PackageCommandError, PackageQualificationLimits,
            qualify_package_archives,
        };
        use ic_build_host_process::tool::{OutputLimit, OutputLimits, ToolError};
        use std::{path::PathBuf, process::Command};
        let args = std::env::args_os().skip(1).collect::<Vec<_>>();
        if args.len() != 4 {
            eprintln!("expected workspace, archive root, retained evidence and version");
            std::process::exit(2);
        }
        let Some(version) = args[3].to_str() else {
            std::process::exit(2)
        };
        let packages = [
            PackageArchiveSpec {
                name: "ic-blob-storage-contracts",
                version,
                patch_registry: true,
            },
            PackageArchiveSpec {
                name: "ic-blob-storage",
                version,
                patch_registry: false,
            },
        ];
        let result = qualify_package_archives(
            &PathBuf::from(&args[0]),
            &PathBuf::from(&args[1]),
            &PathBuf::from(&args[2]),
            &packages,
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
        println!("Coordinated archive graph, tests and final integrity checks passed.");
    }
}
