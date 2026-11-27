//! Qualify the product schemas against the mounted methods.
mod mounts;
#[path = "../../../../crates/ic-blob-storage/build/schema/mod.rs"]
mod schema;
use quote::quote;
use schema::Names;
use syn::parse_quote;
fn read(path: &str) -> Vec<syn::Item> {
    schema::read(&format!("../../crates/{path}"))
}
pub(super) fn generate() {
    let product = schema::product(
        "../../crates/ic-blob-storage",
        &parse_quote!(ic_blob_storage),
    );
    let app = schema::dtos(
        "../../crates/ic-blob-storage-reference-app",
        "ic-blob-storage-reference-app/src/protocol.rs",
        &parse_quote!(ic_blob_storage_reference_app::protocol),
    );
    let mounts = mounts::generate();
    let out = std::env::var_os("OUT_DIR").unwrap();
    std::fs::write(
        std::path::Path::new(&out).join("contracts.rs"),
        quote!(#product #app #mounts).to_string(),
    )
    .unwrap();
}
