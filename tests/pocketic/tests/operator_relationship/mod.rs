//! Query-only relationship inspection over a controlled local source, not Cashier.
use super::*;
use blob_test_protocol::balance::BalanceSourceConfig;
use ic_blob_storage::ops::caffeine::{
    query::{CashierQuery, CashierQueryRequest, reply::BoundRelationshipReplyError},
    relationship::{
        PaymentRelationshipBinding, PaymentRelationshipProviderError, PaymentRelationshipReply,
        PaymentRelationshipReplyError, PaymentRelationshipReplyLimits,
    },
};
use ic_testkit::pocket_ic::common::rest::BlobCompression;
use std::num::NonZeroUsize;

fn owner() -> Principal {
    Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}
fn payer() -> Principal {
    Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()
}
fn limits() -> PaymentRelationshipReplyLimits {
    PaymentRelationshipReplyLimits {
        max_bytes: NonZeroUsize::new(4096).unwrap(),
        decoding_quota: NonZeroUsize::new(100_000).unwrap(),
        skipping_quota: NonZeroUsize::new(1000).unwrap(),
        max_type_entries: NonZeroUsize::new(32).unwrap(),
    }
}
fn bytes(name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/ic-blob-storage/tests/fixtures/caffeine-relationship")
        .join(format!("{name}.hex"));
    let hex = std::fs::read_to_string(path).unwrap();
    hex.trim()
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
        .collect()
}

impl Fixture {
    fn relationship_request(
        &self,
        paid_canister: Principal,
        payment_account: Principal,
    ) -> CashierQueryRequest {
        CashierQueryRequest::new(
            self.gateway,
            CashierQuery::PaymentRelationship(PaymentRelationshipBinding {
                paid_canister,
                payment_account,
            }),
        )
        .unwrap()
    }
    fn relationship_response(&self, bytes: Vec<u8>, hold: bool, reject: bool) {
        // The existing bounded raw-response slot is shared by the local experiments.
        let configured: bool = self
            .harness
            .pic
            .update_candid_as(
                self.gateway,
                self.driver,
                "configure_balance",
                (BalanceSourceConfig {
                    account: owner(),
                    bytes,
                    hold,
                    reject,
                },),
            )
            .unwrap();
        assert!(configured);
    }
    fn inspect_relationship(
        &self,
        request: &CashierQueryRequest,
    ) -> Result<PaymentRelationshipReply, BoundRelationshipReplyError> {
        let before = self.journals();
        let bytes = self
            .harness
            .pic
            .query_call(
                request.cashier(),
                self.driver,
                request.method_name(),
                request.arguments().to_vec(),
            )
            .unwrap();
        let result = request.decode_relationship_reply(request.cashier(), &bytes, limits());
        assert!(
            self.journals() == before,
            "inspection preserves all journals"
        );
        result
    }
}

#[test]
fn relationship_query_preserves_exact_fields_absence_and_provider_failures_without_mutation() {
    let f = Fixture::new();
    let request = f.relationship_request(owner(), payer());
    f.relationship_response(bytes("wide-signed"), false, false);
    let PaymentRelationshipReply::ReportedRelationship(report) =
        f.inspect_relationship(&request).unwrap()
    else {
        panic!("report expected")
    };
    assert_eq!(
        report.binding(),
        PaymentRelationshipBinding {
            paid_canister: owner(),
            payment_account: payer()
        }
    );
    assert_eq!(
        report.spending_limit_per_day(),
        &candid::Int::parse(b"-340282366920938463463374607431768211456").unwrap()
    );
    assert_eq!(
        report.current_period_spent(),
        &candid::Int::parse(b"340282366920938463463374607431768211456").unwrap()
    );
    assert_eq!(report.expiration_timestamp(), None);
    for (name, expected) in [
        ("absent", PaymentRelationshipReply::NoRelationshipReported),
        (
            "not-found",
            PaymentRelationshipReply::ProviderFailure(
                PaymentRelationshipProviderError::RelationshipNotFound(owner()),
            ),
        ),
        (
            "denied",
            PaymentRelationshipReply::ProviderFailure(
                PaymentRelationshipProviderError::NotAuthorized(payer()),
            ),
        ),
        (
            "invalid",
            PaymentRelationshipReply::ProviderFailure(
                PaymentRelationshipProviderError::InvalidRequest,
            ),
        ),
        (
            "internal",
            PaymentRelationshipReply::ProviderFailure(
                PaymentRelationshipProviderError::InternalError,
            ),
        ),
    ] {
        f.relationship_response(bytes(name), false, false);
        assert_eq!(f.inspect_relationship(&request), Ok(expected));
    }
    assert_eq!(
        request.query(),
        CashierQuery::PaymentRelationship(PaymentRelationshipBinding {
            paid_canister: owner(),
            payment_account: payer()
        })
    );
}

#[test]
fn mismatched_or_unusable_relationship_replies_never_select_a_fallback_payer() {
    let f = Fixture::new();
    let request = f.relationship_request(owner(), payer());
    f.relationship_response(bytes("linked"), false, false);
    let wrong_payer = f.relationship_request(owner(), owner());
    assert_eq!(
        f.inspect_relationship(&wrong_payer),
        Err(BoundRelationshipReplyError::Reply(
            PaymentRelationshipReplyError::PaymentAccountMismatch
        ))
    );
    // Configure an otherwise valid report for another requested owner. The source
    // accepts that request but the shared decoder rejects the returned owner.
    let configured: bool = f
        .harness
        .pic
        .update_candid_as(
            f.gateway,
            f.driver,
            "configure_balance",
            (BalanceSourceConfig {
                account: payer(),
                bytes: bytes("linked"),
                hold: false,
                reject: false,
            },),
        )
        .unwrap();
    assert!(configured);
    assert_eq!(
        f.inspect_relationship(&f.relationship_request(payer(), payer())),
        Err(BoundRelationshipReplyError::Reply(
            PaymentRelationshipReplyError::PaidCanisterMismatch
        ))
    );
    for (bytes, error) in [
        (vec![], PaymentRelationshipReplyError::InvalidReply),
        (vec![0; 4097], PaymentRelationshipReplyError::ReplyTooLarge),
    ] {
        f.relationship_response(bytes, false, false);
        assert_eq!(
            f.inspect_relationship(&request),
            Err(BoundRelationshipReplyError::Reply(error))
        );
    }
}

#[test]
fn query_cannot_bypass_driver_owner_or_source_hold_controls() {
    let f = Fixture::new();
    let request = f.relationship_request(owner(), payer());
    f.relationship_response(bytes("linked"), false, false);
    let before = f.journals();
    for caller in [Principal::anonymous(), f.authority, Fake::principal(99)] {
        assert!(
            f.harness
                .pic
                .query_call(
                    f.gateway,
                    caller,
                    request.method_name(),
                    request.arguments().to_vec()
                )
                .is_err()
        );
    }
    let wrong_owner = f.relationship_request(payer(), payer());
    assert!(
        f.harness
            .pic
            .query_call(
                f.gateway,
                f.driver,
                wrong_owner.method_name(),
                wrong_owner.arguments().to_vec()
            )
            .is_err()
    );
    assert_eq!(f.journals(), before);
    for (hold, reject) in [(true, false), (false, true)] {
        f.relationship_response(bytes("linked"), hold, reject);
        let before = f.journals();
        assert!(
            f.harness
                .pic
                .query_call(
                    f.gateway,
                    f.driver,
                    request.method_name(),
                    request.arguments().to_vec()
                )
                .is_err()
        );
        assert_eq!(f.journals(), before);
    }
}

#[test]
fn old_stable_bytes_do_not_replace_live_inspection_and_restoration_fences_queries() {
    let f = Fixture::new();
    let request = f.relationship_request(owner(), payer());
    f.relationship_response(bytes("linked"), false, false);
    let old = f.harness.pic.get_stable_memory(f.gateway);
    f.relationship_response(bytes("denied"), false, false);
    f.harness
        .pic
        .set_stable_memory(f.gateway, old, BlobCompression::NoCompression);
    assert_eq!(
        f.inspect_relationship(&request),
        Ok(PaymentRelationshipReply::ProviderFailure(
            PaymentRelationshipProviderError::NotAuthorized(payer())
        ))
    );
    f.upgrade_fixture(f.gateway, true);
    let before = f.journals();
    assert!(
        f.harness
            .pic
            .query_call(
                f.gateway,
                f.driver,
                request.method_name(),
                request.arguments().to_vec()
            )
            .is_err()
    );
    assert_eq!(f.journals(), before);
}
