use super::*;
use serde_json::{Value, json};

pub(in crate::operator::inventory) fn encoded() -> Value {
    let root = "sha256:0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd";
    json!({
        "totals":{"assets":1,"source_bytes":3,"source_chunks":1,"distinct_blobs":1,"distinct_bytes":3,"distinct_chunks":1},
        "assets":[{"asset":"a","source":"never-opened.bin","root":root}],
        "blobs":[{"claim":{"root":root,"bytes":3,"headers":[{"name":"Content-Length","value":"3"},{"name":"Content-Type","value":"text/plain"}]},
            "chunk_hashes":["sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7"],
            "computed_content_digest":"sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"}]
    })
}

pub(in crate::operator::inventory) fn prepared() -> Inventory {
    parse(&serde_json::to_vec(&encoded()).unwrap()).unwrap()
}

#[test]
fn accepts_prepared_inventory_and_preserves_duplicate_asset_demand() {
    let mut value = encoded();
    let mut alias = value["assets"][0].clone();
    alias["asset"] = json!("alias");
    value["assets"].as_array_mut().unwrap().push(alias);
    value["totals"]["assets"] = json!(2);
    value["totals"]["source_bytes"] = json!(6);
    value["totals"]["source_chunks"] = json!(2);
    let inventory = parse(&serde_json::to_vec(&value).unwrap()).unwrap();
    assert_eq!(inventory.blobs[0].assets, ["a", "alias"]);
    assert_eq!(inventory.blobs[0].bytes, 3);
    assert_eq!(inventory.blobs[0].chunks, 1);
}

#[test]
fn rejects_tampered_metadata_leaves_totals_and_asset_bindings() {
    for pointer in [
        "/totals/distinct_bytes",
        "/totals/source_chunks",
        "/blobs/0/claim/bytes",
    ] {
        let mut value = encoded();
        *value.pointer_mut(pointer).unwrap() = json!(99);
        assert!(matches!(
            parse(&serde_json::to_vec(&value).unwrap()),
            Err(Failure::InvalidRequest)
        ));
    }
    for pointer in [
        "/blobs/0/chunk_hashes/0",
        "/blobs/0/claim/root",
        "/assets/0/root",
    ] {
        let mut value = encoded();
        *value.pointer_mut(pointer).unwrap() = json!(format!("sha256:{}", "00".repeat(32)));
        assert!(matches!(
            parse(&serde_json::to_vec(&value).unwrap()),
            Err(Failure::InvalidRequest)
        ));
    }
    let mut value = encoded();
    let duplicate = value["assets"][0].clone();
    value["assets"].as_array_mut().unwrap().push(duplicate);
    assert!(matches!(
        parse(&serde_json::to_vec(&value).unwrap()),
        Err(Failure::InvalidRequest)
    ));
    let mut value = encoded();
    let duplicate = value["blobs"][0].clone();
    value["blobs"].as_array_mut().unwrap().push(duplicate);
    assert!(matches!(
        parse(&serde_json::to_vec(&value).unwrap()),
        Err(Failure::InvalidRequest)
    ));
}

#[test]
fn bounds_input_and_rejects_unknown_or_duplicate_json_fields() {
    assert!(matches!(
        parse(&vec![b' '; usize::try_from(MAX_BYTES).unwrap() + 1]),
        Err(Failure::InvalidRequest)
    ));
    let mut value = encoded();
    value["unexpected"] = json!(true);
    assert!(matches!(
        parse(&serde_json::to_vec(&value).unwrap()),
        Err(Failure::InvalidRequest)
    ));
    let text = serde_json::to_string(&encoded()).unwrap().replacen(
        "\"assets\":1",
        "\"assets\":1,\"assets\":1",
        1,
    );
    assert!(matches!(
        parse(text.as_bytes()),
        Err(Failure::InvalidRequest)
    ));
}
