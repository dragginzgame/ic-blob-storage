use super::*;
use crate::model::gateway::{
    GatewayListError, GatewayListLimits, membership::GatewayMembership, registry::GatewaySyncError,
};
use crate::ops::caffeine::relationship::PaymentRelationshipBinding;
use std::num::{NonZeroU128, NonZeroUsize};

fn p(value: u8) -> Principal {
    Principal::from_slice(&[value, 1])
}
fn owner() -> Principal {
    Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").expect("owner")
}
fn payer() -> Principal {
    Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").expect("payer")
}
fn n(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("positive")
}
fn balance_limits() -> BalanceReplyLimits {
    BalanceReplyLimits {
        max_bytes: n(4096),
        decoding_quota: n(100_000),
        skipping_quota: n(1000),
        max_type_entries: n(32),
    }
}
fn relationship_limits() -> PaymentRelationshipReplyLimits {
    PaymentRelationshipReplyLimits {
        max_bytes: n(4096),
        decoding_quota: n(100_000),
        skipping_quota: n(1000),
        max_type_entries: n(32),
    }
}
fn balance(account: Principal) -> CashierQueryRequest {
    CashierQueryRequest::new(p(1), CashierQuery::Balance { account }).expect("request")
}
fn relationship(paid_canister: Principal, payment_account: Principal) -> CashierQueryRequest {
    CashierQueryRequest::new(
        p(1),
        CashierQuery::PaymentRelationship(PaymentRelationshipBinding {
            paid_canister,
            payment_account,
        }),
    )
    .expect("request")
}
fn fixture(hex: &str) -> Vec<u8> {
    let (pairs, rest) = hex.trim().as_bytes().as_chunks::<2>();
    assert!(rest.is_empty());
    pairs
        .iter()
        .map(|b| u8::from_str_radix(std::str::from_utf8(b).expect("hex"), 16).expect("byte"))
        .collect()
}

fn gateway_limits() -> GatewayReplyLimits {
    GatewayReplyLimits {
        max_bytes: n(4096),
        decoding_quota: n(100_000),
        skipping_quota: n(1000),
        max_type_entries: n(32),
    }
}

fn gateway_registry() -> GatewayRegistry {
    let scope = GatewayScope::new(p(2), NonZeroU128::new(1).unwrap(), p(1)).unwrap();
    let mut membership = GatewayMembership::new(GatewayListLimits {
        max_entries: n(2),
        max_unique: n(2),
    });
    membership.add(p(3)).unwrap();
    GatewayRegistry::new(scope, membership)
}

#[test]
fn gateway_request_and_registry_correlation_precede_payload_processing() {
    let request = CashierQueryRequest::new(p(1), CashierQuery::StorageGateways).unwrap();
    let mut registry = gateway_registry();
    let scope = registry.scope();
    let token = registry.begin_sync().unwrap();
    let before = registry.clone();
    let oversized = vec![0; 4097];
    for wrong_method in [balance(owner()), relationship(owner(), payer())] {
        assert_eq!(
            wrong_method.apply_gateway_sync_reply(
                &mut registry,
                token,
                scope,
                &oversized,
                gateway_limits()
            ),
            Err(QueryReplyBindingError::MethodMismatch.into())
        );
        assert_eq!(registry, before);
    }
    for (response_scope, error) in [
        (
            GatewayScope::new(scope.service(), scope.namespace(), p(9)).unwrap(),
            BoundGatewayReplyError::Binding(QueryReplyBindingError::SourceMismatch),
        ),
        (
            GatewayScope::new(p(9), scope.namespace(), scope.cashier()).unwrap(),
            GatewayReplyError::Sync(GatewaySyncError::WrongScope).into(),
        ),
        (
            GatewayScope::new(
                scope.service(),
                NonZeroU128::new(2).unwrap(),
                scope.cashier(),
            )
            .unwrap(),
            GatewayReplyError::Sync(GatewaySyncError::WrongScope).into(),
        ),
    ] {
        assert_eq!(
            request.apply_gateway_sync_reply(
                &mut registry,
                token,
                response_scope,
                &oversized,
                gateway_limits()
            ),
            Err(error)
        );
        assert_eq!(registry, before);
    }
    registry.cancel_sync(token).unwrap();
    registry.begin_sync().unwrap();
    let newer = registry.clone();
    assert_eq!(
        request.apply_gateway_sync_reply(&mut registry, token, scope, &oversized, gateway_limits()),
        Err(GatewayReplyError::Sync(GatewaySyncError::StaleSync).into())
    );
    assert_eq!(registry, newer);
}

#[test]
fn gateway_request_preserves_rejections_and_consumes_only_a_valid_pending_reply() {
    let request = CashierQueryRequest::new(p(1), CashierQuery::StorageGateways).unwrap();
    let original = request.clone();
    let mut registry = gateway_registry();
    let scope = registry.scope();
    let token = registry.begin_sync().unwrap();
    let pending = registry.clone();
    for (bytes, error) in [
        (vec![], GatewayReplyError::InvalidReply),
        (vec![0; 4097], GatewayReplyError::ReplyTooLarge),
        (
            candid::encode_one(vec![p(4); 3]).unwrap(),
            GatewayReplyError::Sync(GatewaySyncError::InvalidList(
                GatewayListError::TooManyEntries {
                    actual: 3,
                    maximum: 2,
                },
            )),
        ),
    ] {
        assert_eq!(
            request.apply_gateway_sync_reply(&mut registry, token, scope, &bytes, gateway_limits()),
            Err(error.into())
        );
        assert_eq!(registry, pending);
    }
    let bytes = fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-gateway/observed.hex"
    )));
    request
        .apply_gateway_sync_reply(&mut registry, token, scope, &bytes, gateway_limits())
        .unwrap();
    let gateway =
        Principal::from_text("jf2g2-sl4zh-zhvyx-v3zcb-hmumz-2at2q-wuamm-4qiml-43msi-r3man-eqe")
            .unwrap();
    assert_eq!(registry.gateways().principals(), &[gateway]);
    assert_eq!(registry.sync_view().pending_sequence, None);
    let completed = registry.clone();
    assert_eq!(
        request.apply_gateway_sync_reply(&mut registry, token, scope, &bytes, gateway_limits()),
        Err(GatewayReplyError::Sync(GatewaySyncError::StaleSync).into())
    );
    assert_eq!(registry, completed);
    let token = registry.begin_sync().unwrap();
    registry.remove(gateway);
    let revoked = registry.clone();
    assert_eq!(
        request.apply_gateway_sync_reply(&mut registry, token, scope, &bytes, gateway_limits()),
        Err(GatewayReplyError::Sync(GatewaySyncError::StaleSync).into())
    );
    assert_eq!(registry, revoked);
    assert_eq!(request, original);
}

#[test]
fn original_requests_select_the_account_and_both_relationship_parties() {
    let bytes = fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-balance/success.hex"
    )));
    let request = balance(owner());
    let original = request.clone();
    let BalanceReply::ReportedBalance {
        account,
        balance: amounts,
    } = request
        .decode_balance_reply(p(1), &bytes, balance_limits())
        .expect("report")
    else {
        panic!("balance expected")
    };
    assert_eq!(account, owner());
    assert_eq!(amounts.total(), 100);
    assert_eq!(request, original);
    assert_eq!(
        balance(payer()).decode_balance_reply(p(1), &bytes, balance_limits()),
        Err(BoundBalanceReplyError::Reply(
            BalanceReplyError::AccountMismatch
        ))
    );

    let bytes = fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-relationship/linked.hex"
    )));
    let request = relationship(owner(), payer());
    let original = request.clone();
    let PaymentRelationshipReply::ReportedRelationship(view) = request
        .decode_relationship_reply(p(1), &bytes, relationship_limits())
        .expect("report")
    else {
        panic!("relationship expected")
    };
    assert_eq!(
        view.binding(),
        PaymentRelationshipBinding {
            paid_canister: owner(),
            payment_account: payer()
        }
    );
    assert_eq!(request, original);
    for (request, error) in [
        (
            relationship(p(2), payer()),
            PaymentRelationshipReplyError::PaidCanisterMismatch,
        ),
        (
            relationship(owner(), p(2)),
            PaymentRelationshipReplyError::PaymentAccountMismatch,
        ),
    ] {
        assert_eq!(
            request.decode_relationship_reply(p(1), &bytes, relationship_limits()),
            Err(BoundRelationshipReplyError::Reply(error))
        );
    }
}

#[test]
fn method_and_source_mismatches_precede_byte_processing() {
    let balance = balance(owner());
    let relationship = relationship(owner(), payer());
    let gateways = CashierQueryRequest::new(p(1), CashierQuery::StorageGateways).expect("gateways");
    let bytes = [0; 2];
    let balance_limits = BalanceReplyLimits {
        max_bytes: n(1),
        ..balance_limits()
    };
    let relationship_limits = PaymentRelationshipReplyLimits {
        max_bytes: n(1),
        ..relationship_limits()
    };
    for request in [&relationship, &gateways] {
        assert_eq!(
            request.decode_balance_reply(p(2), &bytes, balance_limits),
            Err(QueryReplyBindingError::MethodMismatch.into())
        );
    }
    for request in [&balance, &gateways] {
        assert_eq!(
            request.decode_relationship_reply(p(2), &bytes, relationship_limits),
            Err(QueryReplyBindingError::MethodMismatch.into())
        );
    }
    assert_eq!(
        balance.decode_balance_reply(p(2), &bytes, balance_limits),
        Err(QueryReplyBindingError::SourceMismatch.into())
    );
    assert_eq!(
        relationship.decode_relationship_reply(p(2), &bytes, relationship_limits),
        Err(QueryReplyBindingError::SourceMismatch.into())
    );
    assert_eq!(
        balance.decode_balance_reply(p(1), &bytes, balance_limits),
        Err(BoundBalanceReplyError::Reply(
            BalanceReplyError::ReplyTooLarge
        ))
    );
    assert_eq!(
        relationship.decode_relationship_reply(p(1), &bytes, relationship_limits),
        Err(BoundRelationshipReplyError::Reply(
            PaymentRelationshipReplyError::ReplyTooLarge
        ))
    );
}

#[test]
fn absence_and_provider_failures_do_not_change_the_original_query() {
    let request = relationship(owner(), payer());
    let original = request.clone();
    let bytes = fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-relationship/absent.hex"
    )));
    assert_eq!(
        request.decode_relationship_reply(p(1), &bytes, relationship_limits()),
        Ok(PaymentRelationshipReply::NoRelationshipReported)
    );
    let bytes = fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-relationship/denied.hex"
    )));
    assert!(matches!(
        request.decode_relationship_reply(p(1), &bytes, relationship_limits()),
        Ok(PaymentRelationshipReply::ProviderFailure(_))
    ));
    assert_eq!(request, original);
}
