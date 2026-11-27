//! Generate the optional native inspection schemas from supplier-owned declarations.
#[cfg(feature = "inspection")]
mod schema;

fn main() {
    println!("cargo:rerun-if-changed=build");
    #[cfg(feature = "inspection")]
    {
        let declarations = schema::product(
            ".",
            &syn::Ident::new("crate", proc_macro2::Span::call_site()),
        );
        let out = std::env::var_os("OUT_DIR").unwrap();
        std::fs::write(
            std::path::Path::new(&out).join("inspection.rs"),
            declarations.to_string(),
        )
        .unwrap();
    }
}
