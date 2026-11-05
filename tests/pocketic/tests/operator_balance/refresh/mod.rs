//! Preview and dispatch use typed requests, with no CLI or post-status orchestration.
use super::*;

async fn call(
    f: &Fixture,
    preview: bool,
    caller: Principal,
    request: BalanceRefreshRequest,
) -> Result<(), BalanceFailure> {
    let result = if preview {
        fixture_calls::query_fixture(
            &f.harness.pic,
            request.scope.service,
            caller,
            "preview_balance_refresh",
            (request,),
        )
        .await
    } else {
        fixture_calls::update_fixture(
            &f.harness.pic,
            request.scope.service,
            caller,
            "refresh_balance",
            (request,),
        )
        .await
    };
    let (reply,): (Result<(), BalanceFailure>,) = result.unwrap();
    reply
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn preview_is_passive_and_consumed_refresh_cannot_dispatch_twice() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    f.source_balance(bytes("success"), false, false).await;
    let request = f.refresh_request().await;
    let before = f.journals().await;
    for _ in 0..2 {
        assert_eq!(call(&f, true, f.driver, request).await, Ok(()));
        assert_eq!(f.journals().await, before);
    }
    assert_eq!(call(&f, false, f.driver, request).await, Ok(()));
    assert_eq!(
        f.balance_status().await.balance_observation.usability,
        BalanceUsability::Observed
    );
    let completed = f.journals().await;
    for preview in [true, false] {
        assert_eq!(
            call(&f, preview, f.driver, request).await,
            Err(BalanceFailure::Stale)
        );
        assert_eq!(f.journals().await, completed);
    }
    assert_eq!(f.balance_source_status().await.requests, 1);
    assert_eq!(
        f.balance_status().await.balance_observation.attempts.len(),
        1
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn preview_cannot_authorize_changed_bindings_or_another_callers_refresh() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    f.source_balance(bytes("success"), false, false).await;
    let request = f.refresh_request().await;
    assert_eq!(call(&f, true, f.driver, request).await, Ok(()));
    f.configure_balance(f.balance_scope()).await.unwrap();
    let before = f.journals().await;
    assert_eq!(
        call(&f, false, f.driver, request).await,
        Err(BalanceFailure::Stale)
    );
    let current = f.refresh_request().await;
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
            )
            .await,
            Err(BalanceFailure::Binding)
        );
    }
    assert_eq!(
        call(&f, false, Principal::anonymous(), current).await,
        Err(BalanceFailure::Denied)
    );
    assert_eq!(f.status(Principal::anonymous()).await, None);
    assert_eq!(f.journals().await, before);
    assert_eq!(f.balance_source_status().await.requests, 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn failed_observation_consumes_identity_and_restoration_stays_fenced() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    f.source_balance(bytes("negative-ledger"), false, false)
        .await;
    let request = f.refresh_request().await;
    assert_eq!(
        call(&f, false, f.driver, request).await,
        Err(BalanceFailure::Malformed)
    );
    assert_eq!(
        call(&f, false, f.driver, request).await,
        Err(BalanceFailure::Stale)
    );
    f.upgrade_fixture(f.authority, false).await;
    let before = f.journals().await;
    for preview in [true, false] {
        assert_eq!(
            call(&f, preview, f.driver, f.refresh_request().await).await,
            Err(BalanceFailure::Fenced)
        );
        assert_eq!(f.journals().await, before);
    }
    assert_eq!(f.balance_source_status().await.requests, 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn acknowledgement_and_post_status_failures_never_repeat_an_executed_update() {
    let f = Fixture::new().await;
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
        let service = pic.create_canister().await.unwrap();
        pic.install_canister(
            service,
            super::super::operator_method_mode::update_wasm("refresh_balance", &reply),
            vec![],
            None,
        )
        .await
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
        )
        .await;
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
            .await
            .is_err()
        );
        assert_eq!(pic.get_stable_memory(service).await.unwrap().len(), 65_536);
    }
    let service = pic.create_canister().await.unwrap();
    pic.install_canister(
        service,
        super::super::operator_method_mode::update_wasm("preview_balance_refresh", &[]),
        vec![],
        None,
    )
    .await
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
        .await
        .is_err()
    );
    assert_eq!(
        pic.get_stable_memory(service).await.unwrap(),
        Vec::<u8>::new()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pending_read_blocks_preview_and_refresh_without_dispatching_another_call() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    let request = f.refresh_request().await;
    let held = f.hold_balance().await;
    let pending = f.journals().await;
    for preview in [true, false] {
        assert_eq!(
            call(&f, preview, f.driver, request).await,
            Err(BalanceFailure::Busy)
        );
        assert_eq!(f.journals().await, pending);
    }
    f.resume_balance(held).await.unwrap();
    assert_eq!(
        call(&f, false, f.driver, request).await,
        Err(BalanceFailure::Stale)
    );
    assert_eq!(f.balance_source_status().await.requests, 1);
}
