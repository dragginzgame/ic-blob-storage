use super::*;
use clap::Parser;

fn fixture(text: &str) -> Vec<u8> {
    let text = text.trim();
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn provider_reply_checks_required_storage_identity_and_errors() {
    // Encoded independently by didc 0.6.2 from the reviewed provider's named fields.
    // Error vectors use the complete Cashier AccountInfoGetResult type, not our projection.
    let owner = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let reply = fixture(include_str!("account.hex"));
    let account = decode(&reply, owner).unwrap();
    assert_eq!(account.storage_usage.blob_count, 2);
    assert_eq!(account.storage_usage.blob_size_bytes, 100);
    assert_eq!(account.storage_usage.chunk_count, 3);
    assert_eq!(account.storage_usage.chunk_size_bytes, 200);
    assert_eq!(account.cycle_balance.to_string(), "-123");
    assert_eq!(account.last_charge_ts_ns, 42);
    assert!(decode(&reply, Principal::from_slice(&[1, 1])).is_err());
    for bad in [
        fixture(include_str!("not-found.hex")),
        fixture(include_str!("denied.hex")),
        fixture(include_str!("missing-storage.hex")),
        reply[..reply.len() - 1].to_vec(),
        vec![0; 65_537],
    ] {
        assert!(decode(&bad, owner).is_err());
    }
    let mut trailing = reply;
    trailing.push(0);
    assert!(decode(&trailing, owner).is_err());
}

fn enrollment(maximum: &str) -> crate::Cli {
    crate::Cli::try_parse_from([
        "blob",
        "provider",
        "prepare-enrollment",
        "--url",
        "https://icp-api.io",
        "--cashier",
        "72ch2-fiaaa-aaaar-qbsvq-cai",
        "--ledger",
        "um5iw-rqaaa-aaaaq-qaaba-cai",
        "--payer",
        "rrkah-fqaaa-aaaaa-aaaaq-cai",
        "--owner",
        "ryjl3-tyaaa-aaaaa-aaaba-cai",
        "--deposit-subaccount",
        &"01".repeat(32),
        "--deposit-cycles",
        "100",
        "--ledger-fee-cycles",
        "10",
        "--max-debit-cycles",
        maximum,
        "--daily-limit",
        "1",
        "--expires-at-ns",
        "123",
    ])
    .unwrap()
}

#[test]
fn offline_proposal_checks_total_debit_and_never_loads_an_identity() {
    let crate::Command::Provider(crate::ProviderCommand::PrepareEnrollment(input)) =
        enrollment("110").command
    else {
        panic!()
    };
    assert!(prepare(&input).unwrap().contains("source debit: 110"));
    let crate::Command::Provider(crate::ProviderCommand::PrepareEnrollment(input)) =
        enrollment("109").command
    else {
        panic!()
    };
    assert!(prepare(&input).is_err());
    let crate::Command::Provider(crate::ProviderCommand::PrepareEnrollment(mut input)) =
        enrollment("110").command
    else {
        panic!()
    };
    input.deposit_cycles = NonZeroU64::MAX;
    assert!(prepare(&input).is_err());
    assert!(subaccount(&"é".repeat(32)).is_err());
    assert!(subaccount(&"gg".repeat(32)).is_err());
    assert!(principal("2vxsx-fae").is_err());
}
