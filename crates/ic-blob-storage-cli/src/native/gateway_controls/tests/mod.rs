use super::*;

fn input(action: Action) -> Input {
    Input {
        scope: OperatorScope {
            service: Principal::self_authenticating([1]),
            namespace: u128::MAX,
            cashier: Principal::self_authenticating([2]),
            payment_account: Principal::self_authenticating([3]),
        },
        action,
        directory: "unused".into(),
    }
}
fn arguments(command: &str) -> Vec<String> {
    let scope = input(Action::Sync).scope;
    [
        command,
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "absent.pem",
        "--operator",
        &scope.service.to_text(),
        "--service",
        &scope.service.to_text(),
        "--namespace",
        &scope.namespace.to_string(),
        "--cashier",
        &scope.cashier.to_text(),
        "--payer",
        &scope.payment_account.to_text(),
        "--run-dir",
        "unused",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[test]
fn decisions_require_explicit_exact_cancellation_or_gateway_without_other_action_flags() {
    assert!(matches!(
        Options::parse(&arguments("sync-gateways")).unwrap().command,
        super::super::arguments::Command::GatewayControl(Input {
            action: Action::Sync,
            ..
        })
    ));
    let mut args = arguments("cancel-gateway-sync");
    assert!(matches!(Options::parse(&args), Err(Failure::Arguments)));
    for bad in ["0", "01", "-1", "18446744073709551616"] {
        let mut bad_args = args.clone();
        bad_args.extend(["--sequence".into(), bad.into()]);
        assert!(matches!(Options::parse(&bad_args), Err(Failure::Arguments)));
    }
    args.extend(["--sequence".into(), u64::MAX.to_string()]);
    assert!(matches!(
        Options::parse(&args).unwrap().command,
        super::super::arguments::Command::GatewayControl(Input {
            action: Action::Cancel(u64::MAX),
            ..
        })
    ));
    for command in ["sync-gateways", "revoke-gateway"] {
        let mut args = arguments(command);
        args.extend(["--sequence".into(), "1".into()]);
        assert!(matches!(Options::parse(&args), Err(Failure::Arguments)));
    }
    let mut args = arguments("revoke-gateway");
    args.extend([
        "--gateway".into(),
        Principal::self_authenticating([4]).to_text(),
    ]);
    assert!(matches!(
        Options::parse(&args).unwrap().command,
        super::super::arguments::Command::GatewayControl(Input {
            action: Action::Revoke(_),
            ..
        })
    ));
    args.pop();
    args.push(Principal::anonymous().to_text());
    assert!(matches!(Options::parse(&args), Err(Failure::Arguments)));
    let mut args = arguments("sync-gateways");
    args.pop();
    args.pop();
    assert!(matches!(Options::parse(&args), Err(Failure::Arguments)));
}
#[test]
fn acknowledgments_validate_scope_principal_sequence_and_decoding_bounds() {
    let sync = input(Action::Sync);
    let mut reply = GatewaySyncResponse {
        scope: sync.scope,
        sequence: u64::MAX,
    };
    let bytes = candid::encode_one(Ok::<_, SyncError>(reply)).unwrap();
    assert_eq!(
        acknowledgment(&sync, &bytes).unwrap()["sequence"],
        u64::MAX.to_string()
    );
    reply.scope.namespace = 1;
    assert_eq!(
        acknowledgment(
            &sync,
            &candid::encode_one(Ok::<_, SyncError>(reply)).unwrap()
        ),
        Err(Failure::Binding)
    );
    reply.scope = sync.scope;
    reply.sequence = 0;
    assert_eq!(
        acknowledgment(
            &sync,
            &candid::encode_one(Ok::<_, SyncError>(reply)).unwrap()
        ),
        Err(Failure::InvalidReply)
    );
    let cancel = input(Action::Cancel(u64::MAX));
    assert_eq!(
        acknowledgment(
            &cancel,
            &candid::encode_one(Ok::<_, SyncError>(())).unwrap()
        )
        .unwrap()["sequence"],
        u64::MAX.to_string()
    );
    let gateway = Principal::self_authenticating([4]);
    let revoke = input(Action::Revoke(gateway));
    let mut reply = GatewayRevocationResponse {
        request: GatewayRevocationRequest {
            scope: revoke.scope,
            gateway,
        },
        removed: false,
    };
    assert_eq!(
        acknowledgment(
            &revoke,
            &candid::encode_one(Ok::<_, RevokeError>(reply)).unwrap()
        )
        .unwrap()["removed"],
        false
    );
    reply.request.gateway = sync.scope.cashier;
    assert_eq!(
        acknowledgment(
            &revoke,
            &candid::encode_one(Ok::<_, RevokeError>(reply)).unwrap()
        ),
        Err(Failure::Binding)
    );
    for i in [sync, cancel, revoke] {
        assert_eq!(acknowledgment(&i, &[0]), Err(Failure::InvalidReply));
        assert_eq!(acknowledgment(&i, &vec![0; 4097]), Err(Failure::ReplyLimit));
    }
}
#[test]
fn exact_wire_arguments_and_typed_refusals_preserve_current_contract() {
    for action in [
        Action::Sync,
        Action::Cancel(u64::MAX),
        Action::Revoke(Principal::self_authenticating([4])),
    ] {
        let i = input(action);
        match action {
            Action::Sync => assert_eq!(
                candid::decode_one::<OperatorScope>(&i.argument().unwrap()).unwrap(),
                i.scope
            ),
            Action::Cancel(sequence) => assert_eq!(
                candid::decode_one::<GatewaySyncCancellation>(&i.argument().unwrap()).unwrap(),
                GatewaySyncCancellation {
                    scope: i.scope,
                    sequence
                }
            ),
            Action::Revoke(gateway) => assert_eq!(
                candid::decode_one::<GatewayRevocationRequest>(&i.argument().unwrap()).unwrap(),
                GatewayRevocationRequest {
                    scope: i.scope,
                    gateway
                }
            ),
        }
    }
    let sync = input(Action::Sync);
    assert_eq!(
        acknowledgment(
            &sync,
            &candid::encode_one(Err::<GatewaySyncResponse, _>(SyncError::Busy)).unwrap()
        ),
        Err(Failure::GatewaySyncRefused(SyncError::Busy))
    );
    let revoke = input(Action::Revoke(Principal::self_authenticating([4])));
    assert_eq!(
        acknowledgment(
            &revoke,
            &candid::encode_one(Err::<GatewayRevocationResponse, _>(RevokeError::Fenced)).unwrap()
        ),
        Err(Failure::GatewayRevocationRefused(RevokeError::Fenced))
    );
}
#[test]
fn pending_uncertainty_refusal_and_acknowledgment_never_grant_retry_or_settlement() {
    for (result, label) in [
        (Ok(None), "pending"),
        (Ok(Some(json!({"sequence":"1"}))), "acknowledged"),
        (Err(Failure::Transport), "uncertain"),
        (Err(Failure::Binding), "uncertain"),
        (
            Err(Failure::GatewaySyncRefused(SyncError::Conflict)),
            "refused",
        ),
    ] {
        let value = outcome(&input(Action::Sync), "id", &result);
        assert_eq!(value["outcome"], label);
        assert_eq!(value["retry_authorized"], false);
        assert_eq!(value["provider_deletion"], "not_established");
        assert_eq!(value["billing_cessation"], "not_established");
        assert_eq!(value["membership"], "not_observed");
        assert_eq!(value["historical_receipt"], "not_available");
    }
}
