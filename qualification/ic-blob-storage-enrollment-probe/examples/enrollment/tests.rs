use super::*;

#[test]
fn offline_frames_match_host_types_and_refuse_wrong_direction() {
    let config = Installation {
        operator: Principal::from_slice(&[1, 1]),
        cashier: Principal::from_slice(&[2, 1]),
        target_balance: 100,
    };
    let prepared = run(Command::Prepare {
        operator: config.operator,
        cashier: config.cashier,
        target_balance: config.target_balance,
    })
    .unwrap();
    let init = unhex(prepared["init_hex"].as_str().unwrap()).unwrap();
    let decoded: Installation = wire::decode(
        &init,
        wire::Encoding::Frame {
            method: wire::method_id("enrollment_init"),
            direction: wire::Direction::Request,
        },
        LIMITS,
    )
    .unwrap();
    assert_eq!(decoded, config);
    let request = unhex(prepared["inspect_hex"].as_str().unwrap()).unwrap();
    assert!(
        run(Command::Decode {
            method: Method::Inspect,
            reply_hex: Reply(request)
        })
        .is_err()
    );
    let denied = Err::<View, _>(Error::Denied);
    let bytes = wire::encode(
        &denied,
        wire::Encoding::Frame {
            method: wire::method_id("enrollment_inspect"),
            direction: wire::Direction::Response,
        },
        LIMITS,
    )
    .unwrap();
    assert_eq!(
        run(Command::Decode {
            method: Method::Inspect,
            reply_hex: Reply(bytes.clone())
        })
        .unwrap(),
        json!({ "Err": "Denied" })
    );
    assert!(
        run(Command::Decode {
            method: Method::Attempt,
            reply_hex: Reply(bytes.clone())
        })
        .is_err()
    );
    let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    let command = Command::try_parse_from([
        "enrollment",
        "decode",
        "--method",
        "inspect",
        "--reply-hex",
        &hex,
    ])
    .unwrap();
    assert_eq!(run(command).unwrap(), json!({ "Err": "Denied" }));
}

#[test]
fn clap_rejects_invalid_principals_targets_and_hex() {
    let payer = "o5trf-oqyg7-cawjp-xs4pw-aomb3-iwki5-hyezf-qahfz-j3ffd-jh4fc-oqe";
    let cashier = "72ch2-fiaaa-aaaar-qbsvq-cai";
    for (operator, target) in [
        ("2vxsx-fae", "1"),
        ("aaaaa-aa", "1"),
        (payer, "0"),
        (payer, "9900000001"),
    ] {
        assert!(
            Command::try_parse_from([
                "enrollment",
                "prepare",
                "--operator",
                operator,
                "--cashier",
                cashier,
                "--target-balance",
                target
            ])
            .is_err()
        );
    }
    for value in ["0", "zz", "éé"] {
        assert!(unhex(value).is_err());
    }
    assert!(unhex(&"00".repeat(LIMITS.bytes + 1)).is_err());
}

#[test]
fn retained_cashier_refusal_is_distinct_from_transport_success() {
    // Actual 2026-11-15 owner-origin account_top_up_v1 reply. No signed envelope,
    // identity material or account identifiers are present in this provider refusal.
    let hex = include_str!("fixtures/top-up-without-cycles.hex").trim();
    let command =
        Command::try_parse_from(["enrollment", "decode-top-up", "--reply-hex", hex]).unwrap();
    assert_eq!(
        run(command).unwrap(),
        json!({
            "provider_result": "Err", "error": "TopUpWithoutCycles"
        })
    );
    let bytes = unhex(hex).unwrap();
    assert!(cashier::classify(&bytes[..bytes.len() - 1]).is_err());
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(cashier::classify(&trailing).is_err());
    assert!(cashier::classify(&vec![0; 8193]).is_err());
    assert!(cashier::classify(b"DIDL\0\0").is_err());
    // A valid Candid value with another method's result shape must not be success.
    // No type table, one nat64 argument (0x78), little-endian 42.
    let wrong_type = b"DIDL\x00\x01\x78\x2a\x00\x00\x00\x00\x00\x00\x00";
    assert!(cashier::classify(wrong_type).is_err());
}
