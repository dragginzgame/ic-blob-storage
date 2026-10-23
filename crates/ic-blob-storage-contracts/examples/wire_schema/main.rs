//! Emit host-only metadata or TypeScript source to stdout; owns no transport.
use ic_blob_storage_contracts::wire::schema;

fn main() {
    let output = match std::env::args().nth(1).as_deref() {
        Some("metadata") => {
            serde_json::to_string_pretty(&schema::methods().expect("qualified schemas"))
                .expect("JSON")
        }
        Some("client") => schema::typescript("./blob-runtime.ts").expect("qualified TypeScript"),
        Some("adapter") => schema::runtime_adapter("./wire-runtime.ts"),
        Some("runtime") => ic_wire::typescript::RUNTIME.to_owned(),
        _ => panic!("usage: wire_schema metadata|client|adapter|runtime"),
    };
    println!("{output}");
}
