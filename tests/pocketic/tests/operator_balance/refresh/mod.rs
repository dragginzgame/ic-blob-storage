//! Preview and dispatch use typed requests, with no CLI or post-status orchestration.
use super::*;

fn call(
    f: &Fixture,
    preview: bool,
    caller: Principal,
    request: BalanceRefreshRequest,
) -> Result<(), BalanceFailure> {
    let call = if preview {
        fixture_calls::query_fixture
    } else {
        fixture_calls::update_fixture
    };
    let (reply,): (Result<(), BalanceFailure>,) = call(
        &f.harness.pic,
        request.scope.service,
        caller,
        if preview {
            "preview_balance_refresh"
        } else {
            "refresh_balance"
        },
        (request,),
    )
    .unwrap();
    reply
}

#[test]
fn preview_is_passive_and_consumed_refresh_cannot_dispatch_twice() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.source_balance(bytes("success"), false, false);
    let request = f.refresh_request();
    let before = f.journals();
    for _ in 0..2 {
        assert_eq!(call(&f, true, f.driver, request), Ok(()));
        assert_eq!(f.journals(), before);
    }
    assert_eq!(call(&f, false, f.driver, request), Ok(()));
    assert_eq!(
        f.balance_status().balance_observation.usability,
        BalanceUsability::Observed
    );
    let completed = f.journals();
    for preview in [true, false] {
        assert_eq!(
            call(&f, preview, f.driver, request),
            Err(BalanceFailure::Stale)
        );
        assert_eq!(f.journals(), completed);
    }
    assert_eq!(f.balance_source_status().requests, 1);
    assert_eq!(f.balance_status().balance_observation.attempts.len(), 1);
}

#[test]
fn preview_cannot_authorize_changed_bindings_or_another_callers_refresh() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.source_balance(bytes("success"), false, false);
    let request = f.refresh_request();
    assert_eq!(call(&f, true, f.driver, request), Ok(()));
    f.configure_balance(f.balance_scope()).unwrap();
    let before = f.journals();
    assert_eq!(
        call(&f, false, f.driver, request),
        Err(BalanceFailure::Stale)
    );
    let current = f.refresh_request();
    for scope in [
        BalanceScope {
            account: fake_principal(30),
            ..current.scope
        },
        BalanceScope {
            namespace: 2,
            ..current.scope
        },
    ] {
        assert_eq!(
            call(
                &f,
                false,
                f.driver,
                BalanceRefreshRequest { scope, ..current }
            ),
            Err(BalanceFailure::Binding)
        );
    }
    assert_eq!(
        call(&f, false, Principal::anonymous(), current),
        Err(BalanceFailure::Denied)
    );
    assert_eq!(f.status(Principal::anonymous()), None);
    assert_eq!(f.journals(), before);
    assert_eq!(f.balance_source_status().requests, 0);
}

#[test]
fn failed_observation_consumes_identity_and_restoration_stays_fenced() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.source_balance(bytes("negative-ledger"), false, false);
    let request = f.refresh_request();
    assert_eq!(
        call(&f, false, f.driver, request),
        Err(BalanceFailure::Malformed)
    );
    assert_eq!(
        call(&f, false, f.driver, request),
        Err(BalanceFailure::Stale)
    );
    f.upgrade_fixture(f.authority, false);
    let before = f.journals();
    for preview in [true, false] {
        assert_eq!(
            call(&f, preview, f.driver, f.refresh_request()),
            Err(BalanceFailure::Fenced)
        );
        assert_eq!(f.journals(), before);
    }
    assert_eq!(f.balance_source_status().requests, 1);
}

#[test]
fn acknowledgement_and_post_status_failures_never_repeat_an_executed_update() {
    let f = Fixture::new();
    let pic = &f.harness.pic;
    for acknowledged in [true, false] {
        let reply = if acknowledged {
            blob_test_protocol::fixture_wire::encode_reply(
                "refresh_balance",
                Ok::<(), BalanceFailure>(()),
            )
            .unwrap()
        } else {
            b"bad".to_vec()
        };
        let service = pic.create_canister().unwrap();
        pic.install_canister(
            service,
            super::super::operator_method_mode::update_wasm("refresh_balance", &reply),
            vec![],
            None,
        )
        .unwrap();
        let request = BalanceRefreshRequest {
            scope: BalanceScope {
                service,
                ..f.balance_scope()
            },
            revision: 1,
            sequence: 1,
        };
        let result = fixture_calls::update_fixture::<(Result<(), BalanceFailure>,), _>(
            pic,
            service,
            f.driver,
            "refresh_balance",
            (request,),
        );
        if acknowledged {
            assert_eq!(result.unwrap().0, Ok(()));
        } else {
            assert!(
                result.is_err(),
                "malformed acknowledgement is uncertain even though the update ran"
            );
        }
        assert!(
            fixture_calls::query_fixture::<(Option<OperatorStatusView>,), _>(
                pic,
                service,
                f.driver,
                "operator_status",
                ()
            )
            .is_err()
        );
        assert_eq!(pic.get_stable_memory(service).unwrap().len(), 65_536);
    }
    let service = pic.create_canister().unwrap();
    pic.install_canister(
        service,
        super::super::operator_method_mode::update_wasm("preview_balance_refresh", &[]),
        vec![],
        None,
    )
    .unwrap();
    let request = BalanceRefreshRequest {
        scope: BalanceScope {
            service,
            ..f.balance_scope()
        },
        revision: 1,
        sequence: 1,
    };
    assert!(
        fixture_calls::query_fixture::<(Result<(), BalanceFailure>,), _>(
            pic,
            service,
            f.driver,
            "preview_balance_refresh",
            (request,)
        )
        .is_err()
    );
    assert_eq!(pic.get_stable_memory(service).unwrap(), Vec::<u8>::new());
}

#[test]
fn pending_read_blocks_preview_and_refresh_without_dispatching_another_call() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    let request = f.refresh_request();
    let held = f.hold_balance();
    let pending = f.journals();
    for preview in [true, false] {
        assert_eq!(
            call(&f, preview, f.driver, request),
            Err(BalanceFailure::Busy)
        );
        assert_eq!(f.journals(), pending);
    }
    f.resume_balance(held).unwrap();
    assert_eq!(
        call(&f, false, f.driver, request),
        Err(BalanceFailure::Stale)
    );
    assert_eq!(f.balance_source_status().requests, 1);
}
