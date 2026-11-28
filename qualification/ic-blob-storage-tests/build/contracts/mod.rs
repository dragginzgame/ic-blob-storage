//! Qualify the product schemas against the mounted methods.
mod mounts;
#[path = "../schema/mod.rs"]
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
    let snapshot = "../../crates/ic-blob-storage-wire/src/schemas.rs";
    println!("cargo:rerun-if-changed={snapshot}");
    use std::str::FromStr as _;
    let reviewed = std::fs::read_to_string(snapshot).unwrap();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let generated = out.join("blob-schemas.rs");
    std::fs::write(&generated, product.to_string()).unwrap();
    assert!(
        normalized(proc_macro2::TokenStream::from_str(&reviewed).unwrap())
            == normalized(product.clone()),
        "review and refresh the Blob Wire schemas from {}",
        generated.display()
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

// Compare declaration tokens while allowing rustfmt to sort imports and add commas.
fn normalized(stream: proc_macro2::TokenStream) -> Vec<String> {
    use quote::ToTokens;
    let file: syn::File = syn::parse2(stream).unwrap();
    let mut result = Vec::new();
    for item in file.items {
        let syn::Item::Mod(module) = item else {
            panic!("expected a schema module")
        };
        result.push(module.ident.to_string());
        let mut imports = Vec::new();
        for item in module.content.unwrap().1 {
            if let syn::Item::Use(import) = item {
                import_paths("", import.tree, &mut imports);
            } else {
                tokens(item.into_token_stream(), &mut result);
            }
        }
        imports.sort();
        result.extend(imports);
    }
    result
}

fn import_paths(prefix: &str, import: syn::UseTree, result: &mut Vec<String>) {
    use quote::ToTokens;
    match import {
        syn::UseTree::Group(group) => {
            for entry in group.items {
                import_paths(prefix, entry, result);
            }
        }
        syn::UseTree::Path(path) => {
            import_paths(&format!("{prefix}{}::", path.ident), *path.tree, result)
        }
        leaf => result.push(format!("{prefix}{}", leaf.into_token_stream())),
    }
}

fn tokens(stream: proc_macro2::TokenStream, result: &mut Vec<String>) {
    use proc_macro2::TokenTree;
    let mut items: Vec<_> = stream.into_iter().collect();
    if matches!(items.last(), Some(TokenTree::Punct(p)) if p.as_char() == ',') {
        items.pop();
    }
    for token in items {
        if let TokenTree::Group(group) = token {
            result.push(format!("open {:?}", group.delimiter()));
            tokens(group.stream(), result);
            result.push("close".into());
        } else {
            result.push(token.to_string());
        }
    }
}
