//! Shared independent Caffeine vector inputs for local IC probes.
use blob_test_protocol::journey::{JourneyManifest, JourneyUpload};

pub(crate) struct Vector {
    pub upload: JourneyUpload,
    pub manifest: JourneyManifest,
}

fn hash(value: &serde_json::Value) -> [u8; 32] {
    let text = value
        .as_str()
        .expect("hash")
        .strip_prefix("sha256:")
        .expect("prefix");
    std::array::from_fn(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex"))
}

pub(crate) fn source(name: &str) -> serde_json::Value {
    let vectors: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../crates/ic-blob-storage-contracts/tests/fixtures/caffeine-hashing/vectors.json"
    ))
    .expect("independent JS vectors");
    vectors["vectors"]
        .as_array()
        .expect("vectors")
        .iter()
        .find(|v| v["name"] == name)
        .expect("selected vector")
        .clone()
}

pub(crate) fn vector(name: &str, id: u8) -> Vector {
    let v = source(name);
    let bytes = v["bytes"].as_u64().expect("length");
    Vector {
        upload: JourneyUpload {
            id,
            root: hash(&v["provider_root"]),
            digest: hash(&v["raw_digest"]),
            bytes,
        },
        manifest: JourneyManifest {
            chunks: v["chunk_hashes"]
                .as_array()
                .expect("chunks")
                .iter()
                .map(hash)
                .collect(),
            headers: v["headers"]
                .as_array()
                .expect("headers")
                .iter()
                .map(|h| {
                    (
                        h["name"].as_str().expect("name").to_owned(),
                        h["value"].as_str().expect("value").to_owned(),
                    )
                })
                .collect(),
        },
    }
}
