use super::*;

#[test]
fn catalog_is_complete_unique_and_uses_mounted_limits() {
    let catalog = methods().unwrap();
    for row in &catalog {
        let name = row["name"].as_str().unwrap();
        let request = crate::wire::request_limits(name).unwrap();
        let response = crate::wire::response_limits(name).unwrap();
        assert_eq!(row["id"], crate::wire::identity(name).unwrap().to_string());
        for (direction, limits) in [("request", request), ("response", response)] {
            for (field, value) in [
                ("Bytes", limits.bytes),
                ("Allocation", limits.allocation),
                ("Nodes", limits.nodes),
                ("Depth", limits.depth),
            ] {
                assert_eq!(row[format!("{direction}{field}")], value);
            }
        }
        if name == "blob_tenant" {
            assert_eq!(row["requestBytes"], 512);
            assert_eq!(row["responseBytes"], 512);
            assert_eq!(row["responseNodes"], 64);
            assert_eq!(row["responseDepth"], 16);
        }
    }
    let typed = projections().unwrap();
    let mut duplicate = typed.clone();
    duplicate.push(typed[0].clone());
    assert!(validate(&duplicate).is_err());
    let mut reused_id = typed.clone();
    reused_id[1].contract.id = reused_id[0].contract.id;
    assert!(validate(&reused_id).is_err());
    assert!(validate(&typed[1..]).is_err());
    assert!(
        typescript("./blob-runtime.js")
            .unwrap()
            .contains("blob_tenant")
    );
    for external in SERVICE_METHODS
        .iter()
        .filter(|m| m.codec == MethodCodec::Candid)
    {
        assert!(crate::wire::identity(external.name).is_err());
        assert!(catalog.iter().all(|m| m["name"] != external.name));
    }
}

#[test]
fn custom_relationship_projection_uses_canonical_decimal_text() {
    use crate::dto::account::AccountRelationshipView;
    use ic_platform::{
        Principal,
        candid::contract::{Int, Nat},
    };
    let value = AccountRelationshipView {
        paid_canister: Principal::anonymous(),
        payment_account: Principal::management_canister(),
        spending_limit_per_day: Int::from(-1000),
        current_period_spent: Int::from(1_234_567),
        current_period_start: 1,
        added_timestamp: 2,
        expiration_timestamp: Some(3),
        bandwidth_baseline_uploaded: "123456789012345678901234567890".parse::<Nat>().unwrap(),
        bandwidth_baseline_downloaded: Nat::from(0_u32),
        bandwidth_baseline_ts_ns: 4,
    };
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(json["spending_limit_per_day"], "-1_000");
    assert_eq!(json["current_period_spent"], "1_234_567");
    for bad in ["-0", "+1", "1000", "01", "1__000"] {
        let mut input = json.clone();
        input["spending_limit_per_day"] = bad.into();
        assert!(serde_json::from_value::<AccountRelationshipView>(input).is_err());
    }
    let schema = native_schema::<AccountRelationshipView>().unwrap();
    assert_eq!(
        schema["fields"][2]["value"],
        json!({"k":"text", "check":"blobInt"})
    );
    assert_eq!(
        schema["fields"][7]["value"],
        json!({"k":"text", "check":"blobNat"})
    );
}

#[test]
fn custom_decimal_native_decoder_refuses_noncanonical_text() {
    use crate::dto::account::{AccountInspectionFailure, AccountInspectionResponse};
    let cases: Value = serde_json::from_str(include_str!(
        "../../../../tests/wire_golden/vectors/blob_inspect_account-response.json"
    ))
    .unwrap();
    let bytes = hex::decode(
        cases.as_array().unwrap().last().unwrap()["hex"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    let valid: Result<AccountInspectionResponse, AccountInspectionFailure> =
        crate::wire::decode_response(
            "blob_inspect_account",
            &bytes,
            crate::wire::MAX_MESSAGE_BYTES,
        )
        .unwrap();
    assert!(valid.is_ok());
    let offset = bytes.windows(6).position(|b| b == b"-1_000").unwrap();
    for bad in [b"+1_000", b"-10000", b"-0_000"] {
        let mut invalid = bytes.clone();
        invalid[offset..offset + 6].copy_from_slice(bad);
        assert!(
            crate::wire::decode_response::<
                Result<AccountInspectionResponse, AccountInspectionFailure>,
            >(
                "blob_inspect_account",
                &invalid,
                crate::wire::MAX_MESSAGE_BYTES
            )
            .is_err()
        );
    }
}
