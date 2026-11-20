//! Feed the actual Serde declarations and Platform mounts to Wire's derive.
//! No second set of endpoint signatures, struct fields or variant ordinals.

mod imports;
mod mounts;

use imports::Names;
use syn::visit::Visit;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{env, fs, path::Path};
use syn::{
    Attribute, Data, DeriveInput, Item, Path as RustPath, Token, parse_quote,
    punctuated::Punctuated,
};

fn read(path: &str) -> Vec<Item> {
    let path = format!("../../crates/{path}");
    println!("cargo:rerun-if-changed={path}");
    syn::parse_file(&fs::read_to_string(path).unwrap())
        .unwrap()
        .items
}

fn serde_dto(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("derive"))
        .any(|a| {
            let derives = a
                .parse_args_with(Punctuated::<RustPath, Token![,]>::parse_terminated)
                .unwrap();
            ["Serialize", "Deserialize"].iter().all(|name| {
                derives
                    .iter()
                    .any(|p| p.segments.last().unwrap().ident == name)
            })
        })
}

fn project(mut dto: DeriveInput) -> TokenStream {
    let native = dto.ident.clone();
    let schema = format_ident!("{native}Schema");
    dto.ident = schema.clone();
    dto.attrs
        .retain(|a| !a.path().is_ident("derive") && !a.path().is_ident("doc"));
    dto.attrs
        .push(parse_quote!(#[derive(ic_wire::typescript::Wire)]));
    dto.attrs.push(parse_quote!(#[allow(dead_code, reason = "Wire reads the declaration; no projection values are constructed")]));
    dto.vis = parse_quote!(pub(crate));
    let fields: Vec<_> = match &mut dto.data {
        Data::Struct(s) => vec![&mut s.fields],
        Data::Enum(e) => e.variants.iter_mut().map(|v| &mut v.fields).collect(),
        Data::Union(_) => panic!("unsupported union {native}"),
    };
    for group in fields {
        for field in group {
            let ty = &field.ty;
            field.ty = parse_quote!(<#ty as crate::projection::Project>::Schema);
        }
    }
    quote! {
        #dto
        impl crate::projection::Project for #native { type Schema = #schema; }
    }
}

fn dtos(file: &str, location: &RustPath) -> TokenStream {
    let items = read(file);
    let mut names = Names::default();
    let mut declarations = Vec::new();
    let mut native = Vec::new();
    for item in &items {
        let attrs = match item {
            Item::Enum(e) => &e.attrs,
            Item::Struct(s) => &s.attrs,
            _ => continue,
        };
        if serde_dto(attrs) {
            let dto: DeriveInput = syn::parse2(quote!(#item)).unwrap();
            names.visit_derive_input(&dto);
            native.push(dto.ident.clone());
            declarations.push(project(dto));
        }
    }
    for item in &items {
        if let Item::Type(alias) = item
            && names.contains(&alias.ident)
        {
            native.push(alias.ident.clone());
        }
    }
    let imports = names.imports(&items, &location.segments.first().unwrap().ident);
    let module = format_ident!("{}", file.replace(['/', '.', '-'], "_"));
    quote!(mod #module { use #location::{#(#native),*}; #imports #(#declarations)* })
}

pub(super) fn generate() {
    let declarations = [
        dtos(
            "ic-blob-storage/src/local/types.rs",
            &parse_quote!(ic_blob_storage::local),
        ),
        dtos(
            "ic-blob-storage/src/protocol.rs",
            &parse_quote!(ic_blob_storage::protocol),
        ),
        dtos(
            "ic-blob-storage/src/references/mod.rs",
            &parse_quote!(ic_blob_storage::references),
        ),
        dtos(
            "ic-blob-storage/src/caffeine/mod.rs",
            &parse_quote!(ic_blob_storage::caffeine),
        ),
        dtos(
            "ic-blob-storage/src/caffeine/manifest.rs",
            &parse_quote!(ic_blob_storage::caffeine),
        ),
        dtos(
            "ic-blob-storage-reference-app/src/protocol.rs",
            &parse_quote!(ic_blob_storage_reference_app::protocol),
        ),
    ];
    let mounts = mounts::generate();
    let out = env::var_os("OUT_DIR").unwrap();
    fs::write(
        Path::new(&out).join("contracts.rs"),
        quote!(#(#declarations)* #mounts).to_string(),
    )
    .unwrap();
}
