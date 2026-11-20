//! Render the public methods of the two product release Wasms for PR comparison.

mod projection;

use clap::Parser;
use ic_host_artifacts::{Length, methods};
use std::{error::Error, fs::File, io::Write, path::PathBuf};

///
/// Artifacts
/// Cargo-selected release modules and the comparison runner’s output directory.
///

#[derive(Parser)]
struct Artifacts {
    standalone: PathBuf,
    application: PathBuf,
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Artifacts::parse();
    for (label, path) in [
        ("standalone", args.standalone),
        ("reference-app", args.application),
    ] {
        let file = File::open(path)?;
        let length = Length::Exactly(file.metadata()?.len());
        let (methods, _) = methods::inventory(file, length)?;
        let mut output = File::create_new(args.output.join(format!("{label}.txt")))?;
        let mut contracts = mounted(label)?;
        let dir = args.output.join(label);
        std::fs::create_dir(&dir)?;
        for (name, kind) in methods {
            let mode = match kind {
                methods::Kind::Query => "query",
                methods::Kind::Update => "update",
                methods::Kind::CompositeQuery => "composite_query",
            };
            let (mounted_mode, document) = contracts
                .remove(&name)
                .ok_or_else(|| format!("no Wire contract for {label}/{name}"))?;
            if mounted_mode != mode {
                return Err(format!("mode mismatch for {label}/{name}").into());
            }
            if !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
                return Err("invalid method filename".into());
            }
            std::fs::write(dir.join(format!("{name}.json")), document)?;
            writeln!(output, "{mode} {name:?}")?;
        }
        if !contracts.is_empty() {
            return Err(format!("mounts missing in {label} Wasm: {:?}", contracts.keys()).into());
        }
    }

    Ok(())
}

type Contracts = std::collections::BTreeMap<String, (String, String)>;

include!(concat!(env!("OUT_DIR"), "/contracts.rs"));

fn add<I: projection::Project, O: projection::Project>(
    contracts: &mut Contracts,
    name: &str,
    mode: &str,
    id: u64,
    mount_bytes: usize,
    request: ic_wire::Limits,
    reply: ic_wire::Limits,
) -> Result<(), Box<dyn Error>> {
    use ic_wire::typescript::{Method, PayloadLimits, SchemaRegistry};

    let mut registry =
        SchemaRegistry::new(Default::default()).map_err(|e| format!("{name}: {e:?}"))?;
    registry
        .register::<I::Schema>("Request")
        .and_then(|()| registry.register::<O::Schema>("Reply"))
        .map_err(|e| format!("{name}: {e:?}"))?;
    let graph = registry.finish().map_err(|e| format!("{name}: {e:?}"))?;
    let payload_limits =
        |limits: ic_wire::Limits, bytes: usize| -> Result<PayloadLimits, Box<dyn Error>> {
            // A framed unit has an empty payload, so its length is the Wire header size.
            let header = ic_wire::encode(
                &(),
                ic_wire::Encoding::Frame {
                    method: id,
                    direction: ic_wire::Direction::Request,
                },
                limits,
            )?
            .len();
            Ok(PayloadLimits {
                bytes: bytes.checked_sub(header).ok_or("frame exceeds limit")? as u64,
                nodes: limits.nodes as u64,
                depth: limits.depth as u64,
            })
        };
    let method = Method {
        export: name,
        id,
        request: "Request",
        response: "Reply",
        request_limits: payload_limits(request, mount_bytes.min(request.bytes))?,
        response_limits: payload_limits(reply, reply.bytes)?,
    };
    let mut docs = graph
        .api_manifests(&[(name, method)])
        .map_err(|e| format!("{name}: {e:?}"))?;
    if contracts
        .insert(
            name.to_owned(),
            (
                mode.to_owned(),
                docs.remove(name).ok_or("missing manifest")?,
            ),
        )
        .is_some()
    {
        return Err(format!("duplicate mount {name}").into());
    }
    Ok(())
}
