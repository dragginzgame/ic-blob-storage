//! Explicit PocketIC asset admission for Blob's installed qualification.
use ic_testkit::server::{PreparedServer, host_asset, prepare};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    match args.as_slice() {
        [command] if command == "asset" => println!("{}", host_asset()?.url),
        [command, directory] if command == "check" => {
            PreparedServer::check(&PathBuf::from(directory))?;
        }
        [command, archive, directory, gzip] if command == "prepare" => {
            prepare(&PathBuf::from(archive), &PathBuf::from(directory), &PathBuf::from(gzip))?;
        }
        _ => return Err("usage: prepare-server asset | check DIRECTORY | prepare ARCHIVE DIRECTORY ABSOLUTE_GZIP".into()),
    }
    Ok(())
}
