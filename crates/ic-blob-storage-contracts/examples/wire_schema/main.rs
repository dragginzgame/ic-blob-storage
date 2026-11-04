//! Emit host-only metadata or TypeScript source to stdout; owns no transport.
use ic_blob_storage_contracts::wire::schema;

fn cli() -> clap::Command {
    clap::Command::new("wire_schema")
        .about("Emit host-only Wire metadata or TypeScript to stdout")
        .arg(
            clap::Arg::new("mode")
                .required(true)
                .value_parser(["metadata", "client", "adapter", "runtime"]),
        )
}

fn main() {
    let args = cli().get_matches();
    let output = match args.get_one::<String>("mode").map(String::as_str) {
        Some("metadata") => {
            serde_json::to_string_pretty(&schema::methods().expect("qualified schemas"))
                .expect("JSON")
        }
        Some("client") => schema::typescript("./blob-runtime.ts").expect("qualified TypeScript"),
        Some("adapter") => schema::runtime_adapter("./wire-runtime.ts"),
        Some("runtime") => ic_wire::typescript::RUNTIME.to_owned(),
        _ => unreachable!("Clap admits only declared output modes"),
    };
    println!("{output}");
}

#[cfg(test)]
mod tests;
