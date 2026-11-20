//! Read the Platform mount's request, reply, byte cap and Wire codec ID together.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, FnArg, Item, MetaNameValue, PathArguments, ReturnType, Token, punctuated::Punctuated,
};

use super::{Names, read};
use syn::visit::Visit;

pub(super) fn generate() -> TokenStream {
    let hosts = [
        ("standalone", "ic-blob-storage-canister"),
        ("reference-app", "ic-blob-storage-reference-app"),
    ];
    let branches = hosts.map(|(label, package)| {
        let host = read(&format!("{package}/src/lib.rs"))
            .into_iter()
            .find_map(|i| match i {
                Item::Mod(m) if m.ident == "host" => Some(m.content.unwrap().1),
                _ => None,
            })
            .expect("inline Platform mounts");
        let external_codec = (label == "standalone").then(|| read(&format!("{package}/src/codec.rs")));
        let codec = external_codec.as_deref().unwrap_or(&host);
        let request_budget = budget(codec, "decode");
        let reply_budget = budget(codec, "encode");
        let mut budgets = Names::default();
        budgets.visit_expr_path(request_budget);
        budgets.visit_expr_path(reply_budget);
        let mut calls = Vec::new();
        let mut names = Names::default();
        for item in &host {
            let Item::Fn(handler) = item else { continue };
            for attr in &handler.attrs {
                let Some(mode) = attr.path().segments.last().map(|s| s.ident.to_string()) else {
                    continue;
                };
                if !matches!(mode.as_str(), "query" | "update" | "composite_query") {
                    continue;
                }
                let options = attr
                    .parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated)
                    .unwrap();
                let option = |name: &str| {
                    &options
                        .iter()
                        .find(|o| o.path.is_ident(name))
                        .unwrap()
                        .value
                };
                for a in &handler.attrs { names.visit_attribute(a); }
                let name = option("name");
                let bytes = option("max_bytes");
                let codec_id = |key: &str| {
                    let Expr::Path(p) = option(key) else {
                        panic!("expected typed {key} hook");
                    };
                    let hook = p.path.segments.last().unwrap();
                    assert_eq!(hook.ident, key);
                    let PathArguments::AngleBracketed(args) = &hook.arguments else {
                        panic!("missing codec ID");
                    };
                    assert_eq!(args.args.len(), 2);
                    syn::parse2::<syn::Ident>(args.args.last().unwrap().to_token_stream()).unwrap()
                };
                let id = codec_id("decode");
                assert_eq!(
                    id,
                    codec_id("encode"),
                    "mismatched request/reply method IDs"
                );
                let value = codec
                    .iter()
                    .find_map(|i| match i {
                        Item::Const(c) if c.ident == id => Some(&c.expr),
                        _ => None,
                    })
                    .expect("mounted Wire ID");
                assert_eq!(handler.sig.inputs.len(), 1);
                let FnArg::Typed(input) = handler.sig.inputs.first().unwrap() else {
                    panic!("expected a single mounted request");
                };
                let request = input.ty.clone();
                let ReturnType::Type(_, ref reply) = handler.sig.output else {
                    panic!("expected a mounted reply");
                };
                names.visit_type(&request);
                names.visit_type(reply);
                calls.push(
                    quote!(add::<#request, #reply>(&mut contracts, #name, #mode, #value, #bytes, #request_budget, #reply_budget)?;),
                );
                break;
            }
        }
        let product = quote::format_ident!("{}", package.replace('-', "_"));
        let imports = names.imports(&host, &product);
        let budget_imports = budgets.imports(codec, &product);
        quote!(#label => { #budget_imports #imports use ic_wire as wire; #(#calls)* })
    });
    quote! {
        fn mounted(label: &str) -> Result<Contracts, Box<dyn std::error::Error>> {
            let mut contracts = Contracts::new();
            match label { #(#branches)* _ => return Err("unknown host".into()) }
            Ok(contracts)
        }
    }
}

// The current hooks directly delegate to Wire. A different codec needs its own qualification.
fn budget<'a>(items: &'a [Item], hook: &str) -> &'a syn::ExprPath {
    let body = items
        .iter()
        .find_map(|item| match item {
            Item::Fn(f) if f.sig.ident == hook => Some(&f.block.stmts),
            _ => None,
        })
        .expect("mounted codec");
    let [syn::Stmt::Expr(Expr::Call(call), None)] = &body[..] else {
        panic!("expected direct Wire {hook} delegation")
    };
    let Expr::Path(callee) = &*call.func else {
        panic!("expected Wire codec")
    };
    assert_eq!(callee.path.segments.last().unwrap().ident, hook);
    assert_eq!(call.args.len(), 3);
    let Expr::Path(limits) = &call.args[2] else {
        panic!("expected named Wire budget")
    };
    limits
}
