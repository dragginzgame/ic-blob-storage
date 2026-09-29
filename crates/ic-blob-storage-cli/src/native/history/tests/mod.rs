use super::*;
use ic_blob_storage::dto::funding::{FundingHistoryEntry, FundingHistoryPage};

fn scope() -> OperatorScope {
    OperatorScope {
        service: Principal::self_authenticating([1]),
        namespace: u128::MAX,
        cashier: Principal::self_authenticating([2]),
        payment_account: Principal::self_authenticating([3]),
    }
}
fn render(page: &FundingHistoryPage) -> Result<Value, Failure> {
    output(
        page.request,
        &candid::encode_one(Ok::<_, FundingHistoryFailure>(page)).unwrap(),
        Principal::self_authenticating([4]),
        "local",
        "http://127.0.0.1",
    )
}
#[test]
fn history_json_preserves_phases_wide_amounts_and_scoped_continuation() {
    let scope = scope();
    let page = FundingHistoryPage {
        request: FundingHistoryRequest {
            scope,
            cursor: None,
        },
        entries: [
            FundingPhase::Prepared,
            FundingPhase::Uncertain,
            FundingPhase::NotEnqueued,
            FundingPhase::Callback {
                refunded: u128::MAX - 1,
            },
        ]
        .into_iter()
        .enumerate()
        .map(|(i, phase)| FundingHistoryEntry {
            scope,
            operation: u128::MAX - i as u128,
            offered: u128::MAX,
            target_balance: if i == 0 { None } else { Some(u128::MAX) },
            phase,
        })
        .collect(),
        next: Some(FundingHistoryCursor {
            scope,
            before_operation: u128::MAX - 3,
        }),
        fenced: true,
    };
    let value = render(&page).unwrap();
    assert_eq!(value["fenced"], true);
    for (entry, state) in value["entries"].as_array().unwrap().iter().zip([
        "prepared",
        "uncertain",
        "not_enqueued",
        "callback",
    ]) {
        assert_eq!(entry["phase"]["state"], state);
        assert_eq!(entry["offered"], u128::MAX.to_string());
    }
    assert_eq!(value["entries"][0]["target_balance"], Value::Null);
    assert_eq!(
        value["entries"][3]["phase"]["refunded"],
        (u128::MAX - 1).to_string()
    );
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), value["next"].to_string()).unwrap();
    assert_eq!(request(scope, Some(file.path())).unwrap().cursor, page.next);
    for changed in [
        OperatorScope {
            namespace: 1,
            ..scope
        },
        OperatorScope {
            service: scope.cashier,
            ..scope
        },
        OperatorScope {
            cashier: scope.service,
            ..scope
        },
        OperatorScope {
            payment_account: scope.service,
            ..scope
        },
    ] {
        assert_eq!(
            request(changed, Some(file.path())),
            Err(Failure::CursorScope)
        );
    }
    let empty = FundingHistoryPage {
        request: FundingHistoryRequest {
            scope,
            cursor: page.next,
        },
        entries: vec![],
        next: None,
        fenced: true,
    };
    let value = render(&empty).unwrap();
    assert_eq!(value["entries"], json!([]));
    assert_eq!(value["next"], Value::Null);
}

#[test]
fn history_reuses_strict_decoder_and_never_turns_errors_into_empty_pages() {
    let input = FundingHistoryRequest {
        scope: scope(),
        cursor: None,
    };
    let run = |bytes: &[u8]| output(input, bytes, scope().service, "local", "http://127.0.0.1");
    for (error, expected) in [
        (FundingHistoryFailure::Denied, Failure::Denied),
        (FundingHistoryFailure::Internal, Failure::ServiceInternal),
        (FundingHistoryFailure::Binding, Failure::Binding),
        (FundingHistoryFailure::CursorScope, Failure::CursorScope),
        (FundingHistoryFailure::Invalid, Failure::ServiceInvalid),
    ] {
        assert_eq!(
            run(&candid::encode_one(Err::<FundingHistoryPage, _>(error)).unwrap()),
            Err(expected)
        );
    }
    assert_eq!(run(&vec![0; 65537]), Err(Failure::ReplyLimit));
    assert_eq!(run(b"DIDL"), Err(Failure::InvalidReply));
    let forged = FundingHistoryPage {
        request: FundingHistoryRequest {
            scope: OperatorScope {
                namespace: 1,
                ..scope()
            },
            ..input
        },
        entries: vec![],
        next: None,
        fenced: false,
    };
    assert_eq!(
        run(&candid::encode_one(Ok::<_, FundingHistoryFailure>(forged)).unwrap()),
        Err(Failure::Binding)
    );
    let file = tempfile::NamedTempFile::new().unwrap();
    for cursor in [
        json!(null),
        json!({"scope":scope_json(scope()),"before_operation":"01"}),
        json!({"scope":scope_json(scope()),"before_operation":1}),
        json!({"scope":scope_json(scope()),"before_operation":"0"}),
        json!({"scope":scope_json(scope()),"before_operation":"1","extra":true}),
    ] {
        std::fs::write(file.path(), cursor.to_string()).unwrap();
        assert_eq!(request(scope(), Some(file.path())), Err(Failure::Arguments));
    }
    std::fs::write(file.path(), vec![b' '; 2049]).unwrap();
    assert_eq!(request(scope(), Some(file.path())), Err(Failure::File));
}
