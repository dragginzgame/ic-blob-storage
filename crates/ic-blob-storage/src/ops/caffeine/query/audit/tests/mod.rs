use super::*;
use crate::ops::caffeine::{
    audit::{AuditLogProviderError, AuditLogReply, AuditLogReplyError, AuditLogReplyLimits},
    query::{
        CashierQuery, CashierQueryRequest,
        reply::{BoundAuditLogReplyError, QueryReplyBindingError},
    },
};
use std::num::NonZeroUsize;

fn owner() -> Principal {
    Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}
fn cashier() -> Principal {
    Principal::from_slice(&[1, 1])
}
fn input() -> AuditLogQuery {
    AuditLogQuery {
        account: owner(),
        max_entries: NonZeroU64::new(2).unwrap(),
        event_type: None,
        continuation: None,
    }
}
fn encoded(input: AuditLogQuery) -> CashierQueryRequest {
    CashierQueryRequest::new(cashier(), CashierQuery::AuditLog(input)).unwrap()
}
fn fixture(directory: &str, name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(directory)
        .join(format!("{name}.hex"));
    let hex = std::fs::read_to_string(path).unwrap();
    let (pairs, rest) = hex.trim().as_bytes().as_chunks::<2>();
    assert!(rest.is_empty());
    pairs
        .iter()
        .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
        .collect()
}
fn limits(entries: u64) -> AuditLogReplyLimits {
    AuditLogReplyLimits {
        max_bytes: NonZeroUsize::new(4096).unwrap(),
        max_csv_bytes: NonZeroUsize::new(1024).unwrap(),
        max_reported_entries: NonZeroU64::new(entries).unwrap(),
        decoding_quota: NonZeroUsize::new(100_000).unwrap(),
        skipping_quota: NonZeroUsize::new(1000).unwrap(),
        max_type_entries: NonZeroUsize::new(32).unwrap(),
    }
}

#[test]
fn scoped_audit_arguments_match_independent_vectors_for_every_event_and_wide_cursor() {
    let first = encoded(input());
    assert_eq!(first.method_name(), "payment_account_audit_log_get_v1");
    assert_eq!(first.arguments(), fixture("caffeine-query", "audit-first"));
    for (event, name) in [
        (AuditLogEvent::Spend, "Spend"),
        (AuditLogEvent::RelationshipAdd, "RelationshipAdd"),
        (AuditLogEvent::RelationshipUpdate, "RelationshipUpdate"),
        (AuditLogEvent::LedgerDeposit, "LedgerDeposit"),
        (AuditLogEvent::TopUp, "TopUp"),
        (AuditLogEvent::RelationshipRemove, "RelationshipRemove"),
        (AuditLogEvent::UnitCharge, "UnitCharge"),
    ] {
        let query = AuditLogQuery {
            event_type: Some(event),
            continuation: Some(AuditLogCursor {
                account: Some(owner()),
                sequence: 7,
            }),
            ..input()
        };
        let request = encoded(query);
        assert_eq!(
            request.arguments(),
            fixture("caffeine-query", &format!("audit-{name}"))
        );
        assert_eq!(request.query(), CashierQuery::AuditLog(query));
        assert_eq!(request.cashier(), cashier());
    }
    let query = AuditLogQuery {
        max_entries: NonZeroU64::new(u64::MAX).unwrap(),
        continuation: Some(AuditLogCursor {
            account: None,
            sequence: u64::MAX,
        }),
        ..input()
    };
    assert_eq!(
        encoded(query).arguments(),
        fixture("caffeine-query", "audit-wide")
    );
    assert_eq!(encoded(query).query(), CashierQuery::AuditLog(query));
}

#[test]
fn audit_inputs_never_broaden_the_account_or_replace_the_original_filter() {
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            CashierQueryRequest::new(invalid, CashierQuery::AuditLog(input())),
            Err(CashierQueryError::InvalidPrincipal {
                field: CashierQueryPrincipal::Cashier
            })
        );
        assert_eq!(
            CashierQueryRequest::new(
                cashier(),
                CashierQuery::AuditLog(AuditLogQuery {
                    account: invalid,
                    ..input()
                })
            ),
            Err(CashierQueryError::InvalidPrincipal {
                field: CashierQueryPrincipal::AuditAccount
            })
        );
    }
    for wrong in [
        cashier(),
        Principal::anonymous(),
        Principal::management_canister(),
    ] {
        let query = AuditLogQuery {
            continuation: Some(AuditLogCursor {
                account: Some(wrong),
                sequence: 0,
            }),
            ..input()
        };
        assert_eq!(
            CashierQueryRequest::new(cashier(), CashierQuery::AuditLog(query)),
            Err(CashierQueryError::AuditCursorAccountMismatch)
        );
    }
    let original = encoded(input());
    for changed in [
        AuditLogQuery {
            account: cashier(),
            ..input()
        },
        AuditLogQuery {
            max_entries: NonZeroU64::new(1).unwrap(),
            ..input()
        },
        AuditLogQuery {
            event_type: Some(AuditLogEvent::TopUp),
            ..input()
        },
        AuditLogQuery {
            continuation: Some(AuditLogCursor {
                account: None,
                sequence: 0,
            }),
            ..input()
        },
    ] {
        assert_ne!(original.arguments(), encoded(changed).arguments());
        assert_ne!(original.query(), encoded(changed).query());
    }
}

#[test]
fn audit_reply_checks_method_source_and_both_page_bounds_without_changing_request() {
    let request = encoded(input());
    let original = request.clone();
    let oversized = vec![0; 4097];
    for query in [
        CashierQuery::Balance { account: owner() },
        CashierQuery::StorageGateways,
        CashierQuery::PaymentRelationship(
            crate::ops::caffeine::relationship::PaymentRelationshipBinding {
                paid_canister: owner(),
                payment_account: owner(),
            },
        ),
    ] {
        let wrong = CashierQueryRequest::new(cashier(), query).unwrap();
        assert_eq!(
            wrong.decode_audit_log_reply(owner(), &oversized, limits(2)),
            Err(QueryReplyBindingError::MethodMismatch.into())
        );
    }
    assert_eq!(
        request.decode_audit_log_reply(owner(), &oversized, limits(2)),
        Err(QueryReplyBindingError::SourceMismatch.into())
    );
    assert_eq!(
        request.decode_audit_log_reply(cashier(), &oversized, limits(2)),
        Err(AuditLogReplyError::ReplyTooLarge.into())
    );
    let bytes = fixture("caffeine-audit", "page");
    for (requested, allowed) in [(1, 2), (2, 1)] {
        let small = encoded(AuditLogQuery {
            max_entries: NonZeroU64::new(requested).unwrap(),
            ..input()
        });
        assert_eq!(
            small.decode_audit_log_reply(cashier(), &bytes, limits(allowed)),
            Err(AuditLogReplyError::TooManyReportedEntries.into())
        );
    }
    let AuditLogReply::ReportedPage(page) = request
        .decode_audit_log_reply(cashier(), &bytes, limits(2))
        .unwrap()
    else {
        panic!("page")
    };
    assert_eq!(page.reported_entries(), 2);
    assert_eq!(page.continuation().unwrap().account(), Some(owner()));
    assert_eq!(page.csv_content(), "kind,value\nsynthetic,1\nsynthetic,2\n");
    assert_eq!(request, original);
}

#[test]
fn audit_absence_failures_and_opaque_cursors_do_not_authorize_follow_up_or_credit() {
    let request = encoded(input());
    let original = request.clone();
    for (name, error) in [
        (
            "unauthorized",
            AuditLogProviderError::NotAuthorized(owner()),
        ),
        ("invalid-request", AuditLogProviderError::InvalidRequest),
        ("internal", AuditLogProviderError::InternalError),
    ] {
        assert_eq!(
            request.decode_audit_log_reply(cashier(), &fixture("caffeine-audit", name), limits(2)),
            Ok(AuditLogReply::ProviderFailure(error))
        );
    }
    for name in ["empty", "optional-account"] {
        let AuditLogReply::ReportedPage(page) = request
            .decode_audit_log_reply(cashier(), &fixture("caffeine-audit", name), limits(2))
            .unwrap()
        else {
            panic!("page")
        };
        assert_eq!(page.reported_entries(), 0);
        if name == "optional-account" {
            let cursor = page.continuation().unwrap();
            assert_eq!(cursor.account(), None);
            assert_eq!(cursor.sequence(), u64::MAX);
        } else {
            assert_eq!(page.continuation(), None);
            assert!(!page.has_more());
        }
    }
    for (bytes, error) in [
        (vec![], AuditLogReplyError::InvalidReply),
        (
            fixture("caffeine-audit", "missing-continuation"),
            AuditLogReplyError::MissingContinuation,
        ),
    ] {
        assert_eq!(
            request.decode_audit_log_reply(cashier(), &bytes, limits(2)),
            Err(BoundAuditLogReplyError::Reply(error))
        );
    }
    assert_eq!(request, original);
}
