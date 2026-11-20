//! Resolve schema fields and handler signatures using their original Rust imports.

use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;
use syn::{
    Ident, Item, TypePath, UseTree,
    visit::{self, Visit},
};

#[derive(Default)]
pub(super) struct Names(BTreeSet<Ident>);

impl<'ast> Visit<'ast> for Names {
    fn visit_attribute(&mut self, attr: &'ast syn::Attribute) {
        assert!(
            !attr.path().is_ident("cfg") && !attr.path().is_ident("cfg_attr"),
            "target-dependent Wire declarations require a target-qualified projection"
        );
    }

    fn visit_expr_path(&mut self, path: &'ast syn::ExprPath) {
        self.0
            .insert(path.path.segments.first().unwrap().ident.clone());
    }

    fn visit_type_path(&mut self, path: &'ast TypePath) {
        self.0
            .insert(path.path.segments.first().unwrap().ident.clone());
        visit::visit_type_path(self, path);
    }
}

impl Names {
    pub(super) fn imports(&self, items: &[Item], product: &Ident) -> TokenStream {
        let mut result = TokenStream::new();
        for item in items {
            if let Item::Use(u) = item {
                self.import(&u.tree, &TokenStream::new(), product, &mut result);
            }
        }
        result
    }

    fn import(&self, tree: &UseTree, prefix: &TokenStream, product: &Ident, out: &mut TokenStream) {
        match tree {
            UseTree::Path(p) => {
                let name = if p.ident == "crate" {
                    product
                } else {
                    &p.ident
                };
                self.import(&p.tree, &quote!(#prefix #name::), product, out);
            }
            UseTree::Group(group) => {
                for t in &group.items {
                    self.import(t, prefix, product, out);
                }
            }
            UseTree::Name(n) if self.0.contains(&n.ident) => out.extend(quote!(use #prefix #n;)),
            UseTree::Rename(n) if self.0.contains(&n.rename) => out.extend(quote!(use #prefix #n;)),
            UseTree::Glob(_) => {
                panic!("glob imports need explicit resolution before schema export")
            }
            _ => {}
        }
    }

    pub(super) fn contains(&self, ident: &Ident) -> bool {
        self.0.contains(ident)
    }
}
