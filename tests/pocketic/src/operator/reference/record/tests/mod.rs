use super::*;
use candid::Principal;
use serde_json::json;
use std::fs;

pub(super) fn encoded() -> serde_json::Value {
    json!({"schema":1,"scope":"pocketic_fixture","asset":"image-a", "service":Principal::from_slice(&[1,1]).to_text(),
        "tenant":Principal::from_slice(&[2,1]).to_text(),"namespace":"1","upload":"1","object":"1","incarnation":"1",
        "root":format!("sha256:{}","11".repeat(32)),"bytes":3,"reference":u128::MAX.to_string(),"operation":u128::MAX.to_string(),"retain":true})
}

#[test]
fn invalid_or_truncated_input_never_creates_journal_contents() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.json");
    let output = dir.path().join("journal");
    fs::create_dir(&output).unwrap();
    for (field, value) in [
        ("schema", json!(0)),
        ("object", json!("2")),
        ("incarnation", json!("2")),
        ("operation", json!("01")),
        ("namespace", json!("0")),
        ("surprise", json!(true)),
    ] {
        let mut changed = encoded();
        changed[field] = value;
        fs::write(&input, serde_json::to_vec(&changed).unwrap()).unwrap();
        assert_eq!(save(&input, &output).unwrap_err(), Failure::InvalidRequest);
    }
    fs::write(&input, b"{\"schema\":").unwrap();
    assert_eq!(save(&input, &output).unwrap_err(), Failure::InvalidRequest);
    fs::write(&input, vec![b' '; usize::try_from(MAX_BYTES).unwrap() + 1]).unwrap();
    assert_eq!(save(&input, &output).unwrap_err(), Failure::InvalidRequest);
    assert_eq!(fs::read_dir(output).unwrap().count(), 0);
}
