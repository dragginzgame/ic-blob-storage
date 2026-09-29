use super::*;
use ic_blob_storage::dto::operator::*;

fn actor() -> Principal {
    Principal::self_authenticating([42])
}
fn options() -> Vec<String> {
    [
        "status",
        "--network",
        "local",
        "--url",
        "http://127.0.0.1:8080",
        "--identity",
        "operator.pem",
        "--operator",
        &actor().to_text(),
        "--service",
        &actor().to_text(),
        "--namespace",
        &u128::MAX.to_string(),
        "--cashier",
        &actor().to_text(),
        "--payer",
        &actor().to_text(),
        "--root-key",
        "root.der",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
fn change(args: &mut [String], flag: &str, value: &str) {
    let i = args.iter().position(|s| s == flag).unwrap();
    args[i + 1] = value.into();
}

#[test]
fn arguments_require_complete_identity_scope_and_explicit_trust() {
    assert_eq!(
        arguments::Options::parse(&options())
            .unwrap()
            .scope
            .namespace,
        u128::MAX
    );
    for (flag, value) in [
        ("--url", "http://example.com"),
        ("--url", "http://localhost:8080"),
        ("--url", "http://user@127.0.0.1"),
        ("--url", "http://127.0.0.1/path"),
        ("--url", "http://127.0.0.1/?secret"),
        ("--url", "http://127.0.0.1/#fragment"),
        ("--url", "http://127.0.0.1:0"),
        ("--network", "ic"),
        ("--namespace", "01"),
        ("--namespace", "0"),
        ("--namespace", "+1"),
        ("--operator", "2vxsx-fae"),
    ] {
        let mut args = options();
        change(&mut args, flag, value);
        assert!(matches!(
            arguments::Options::parse(&args),
            Err(Failure::Arguments)
        ));
    }
    let mut args = options();
    args.truncate(args.len() - 2);
    assert!(matches!(
        arguments::Options::parse(&args),
        Err(Failure::Arguments)
    ));
    change(&mut args, "--network", "ic");
    change(&mut args, "--url", "https://icp-api.io");
    assert!(arguments::Options::parse(&args).is_ok());
    args.extend(["--operator".into(), actor().to_text()]);
    assert!(matches!(
        arguments::Options::parse(&args),
        Err(Failure::Arguments)
    ));
}

fn status() -> LocalServiceStatus {
    LocalServiceStatus {
        scope: OperatorScope {
            service: actor(),
            namespace: u128::MAX,
            cashier: actor(),
            payment_account: actor(),
        },
        uploads: LocalUploadStatus {
            operations: u64::MAX,
            active_reservations: 0,
            reserved_bytes: 0,
            logical_bytes: 0,
            physical_bytes: 0,
            liability_bytes: u128::MAX,
            fenced: true,
        },
        funding: LocalFundingStatus {
            available_allocation: u128::MAX,
            attachment_allowance: 0,
            transport_accepted: 0,
            refunded: 0,
            not_enqueued: 0,
            reserved_or_uncertain: 0,
            retained_intents: 0,
            intent_capacity: 1,
            last_operation: Some(u128::MAX),
            fenced: true,
        },
        gateways: LocalGatewayStatus {
            members: vec![actor(); 1024],
            last_sequence: u64::MAX,
            pending_sequence: Some(u64::MAX),
            fenced: true,
        },
        reads: LocalReadStatus {
            last_sequence: u64::MAX,
            sessions: 1,
            reserved_bytes: u64::MAX,
            fenced: true,
        },
    }
}
fn encoded(s: &LocalServiceStatus) -> Vec<u8> {
    candid::encode_one(Ok::<_, LocalStatusFailure>(s)).unwrap()
}

#[test]
fn bounded_decode_checks_scope_and_json_preserves_full_width_and_fences() {
    let status = status();
    assert_eq!(
        reply::decode(&encoded(&status), status.scope),
        Ok(status.clone())
    );
    let value = reply::output(&status, actor(), "local", "http://127.0.0.1");
    assert_eq!(
        value["funding"]["available_allocation"],
        u128::MAX.to_string()
    );
    assert_eq!(value["funding"]["last_operation"], u128::MAX.to_string());
    assert_eq!(value["reads"]["reserved_bytes"], u64::MAX.to_string());
    for owner in ["uploads", "funding", "gateways", "reads"] {
        assert_eq!(value[owner]["fenced"], true);
    }
    assert_eq!(
        reply::decode(
            &encoded(&status),
            OperatorScope {
                namespace: 1,
                ..status.scope
            }
        ),
        Err(Failure::Binding)
    );
    for (error, failure) in [
        (LocalStatusFailure::Denied, Failure::Denied),
        (LocalStatusFailure::Binding, Failure::Binding),
        (LocalStatusFailure::Internal, Failure::ServiceInternal),
    ] {
        assert_eq!(
            reply::decode(
                &candid::encode_one(Err::<LocalServiceStatus, _>(error)).unwrap(),
                status.scope
            ),
            Err(failure)
        );
    }
    assert_eq!(
        reply::decode(b"DIDL", status.scope),
        Err(Failure::InvalidReply)
    );
    assert_eq!(
        reply::decode(&vec![0; 65537], status.scope),
        Err(Failure::ReplyTooLarge)
    );
    let mut oversized = status.clone();
    oversized.gateways.members.push(actor());
    assert_eq!(
        reply::decode(&encoded(&oversized), status.scope),
        Err(Failure::InvalidReply)
    );
}

#[test]
fn identity_and_root_files_are_bounded_without_private_diagnostics() {
    let file = tempfile::NamedTempFile::new().unwrap();
    for bytes in [vec![], vec![0; 16385]] {
        std::fs::write(file.path(), bytes).unwrap();
        assert!(matches!(identity(file.path(), actor()), Err(Failure::File)));
    }
    std::fs::write(file.path(), "not a PEM").unwrap();
    assert!(matches!(
        identity(file.path(), actor()),
        Err(Failure::Identity)
    ));
    assert_eq!(read(file.path(), 2), Err(Failure::File));
}
