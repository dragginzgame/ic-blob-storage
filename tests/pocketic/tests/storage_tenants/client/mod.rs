//! Actual canister actor, committed command intent and shared client recovery.
use super::*;
use blob_test_protocol::consumer::{Failure as ConsumerFailure, TenantDispatch};
use ic_blob_storage_pocketic_tests::control_calls::ControlCallExt;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;

mod timeout;

fn consumer_wasm() -> Vec<u8> {
    std::fs::read(fixture_path("BLOB_CONSUMER_PROBE_WASM")).unwrap()
}
async fn fixture(operator_client: bool) -> (Fixture, Principal) {
    let harness = Harness::new().await;
    let driver = fake_principal(2);
    let client = harness
        .pic
        .create_canister_with_settings(
            Some(driver),
            Some(CanisterSettings {
                controllers: Some(vec![driver]),
                ..CanisterSettings::default()
            }),
        )
        .await
        .unwrap();
    let mut f = Fixture::with_operator(
        harness,
        if operator_client {
            client
        } else {
            fake_principal(1)
        },
    )
    .await;
    f.harness
        .pic
        .install_canister(
            client,
            consumer_wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_init", (driver, f.service))
                .unwrap(),
            Some(driver),
        )
        .await
        .unwrap();
    if !operator_client {
        f.tenant = client;
        f.enroll(None, true).await.unwrap();
    }
    (f, client)
}
async fn saved(f: &Fixture, client: Principal) -> Option<TenantUpdateRequest> {
    f.harness
        .pic
        .query_control_as::<Result<Option<TenantUpdateRequest>, ConsumerFailure>, _>(
            client,
            f.controller,
            "fixture_tenant_command",
            (),
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap()
}
async fn inspect(f: &Fixture, client: Principal) -> TenantEnrollmentResponse {
    f.harness
        .pic
        .update_control_as::<Result<TenantEnrollmentResponse, ConsumerFailure>, _>(
            client,
            f.controller,
            "fixture_inspect_tenant",
            f.tenant_scope(),
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap()
}
async fn dispatch(
    f: &Fixture,
    client: Principal,
    input: TenantDispatch,
) -> Result<TenantEnrollmentResponse, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(
            client,
            f.controller,
            "fixture_update_tenant",
            input,
            REPLY_BYTES,
        )
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tenant_control_ingress_refuses_unusable_packets_before_writes() {
    let (f, client) = fixture(true).await;
    let input = TenantDispatch {
        command: TenantUpdateRequest {
            scope: f.tenant_scope(),
            expected: None,
            active: true,
        },
        max_reply_bytes: 4096,
        trap_after_reply: false,
    };
    let valid = ic_blob_storage_contracts::document::encode(input).unwrap();
    let mut trailing = valid.clone();
    trailing.push(0);
    let unknown = ic_blob_storage_contracts::document::encode((input, true)).unwrap();
    let consumer_before = f.harness.pic.get_stable_memory(client).await.unwrap();
    let service_before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for packet in [
        Vec::new(),
        valid[..valid.len() - 1].to_vec(),
        trailing,
        vec![0; 4097],
        unknown,
    ] {
        let rejected = f
            .harness
            .pic
            .update_call(client, f.controller, "fixture_update_tenant", packet)
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&rejected).reject_code,
            RejectCode::CanisterError
        );
        assert_eq!(saved(&f, client).await, None);
        assert_eq!(
            f.harness.pic.get_stable_memory(client).await.unwrap(),
            consumer_before
        );
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            service_before
        );
    }
    assert_eq!(
        dispatch(&f, client, input).await.unwrap(),
        inspect(&f, client).await
    );
    assert_eq!(saved(&f, client).await, Some(input.command));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One replicated-client journey distinguishes usable success from small replies and callback traps"
)]
async fn tenant_client_retains_command_after_success_small_reply_or_callback_trap() {
    for (max_reply_bytes, trap_after_reply) in [(4096, false), (1, false), (4096, true)] {
        let (f, client) = fixture(true).await;
        let command = TenantUpdateRequest {
            scope: f.tenant_scope(),
            expected: None,
            active: true,
        };
        let input = TenantDispatch {
            command,
            max_reply_bytes,
            trap_after_reply,
        };
        assert_eq!(saved(&f, client).await, None);
        let denied: Result<TenantEnrollmentResponse, ConsumerFailure> = f
            .harness
            .pic
            .update_control_as(client, f.other, "fixture_update_tenant", input, REPLY_BYTES)
            .await
            .unwrap();
        assert_eq!(denied, Err(ConsumerFailure::Denied));
        assert_eq!(saved(&f, client).await, None);
        if trap_after_reply {
            let failure = f
                .harness
                .pic
                .update_call(
                    client,
                    f.controller,
                    "fixture_update_tenant",
                    ic_blob_storage_contracts::document::encode(input).unwrap(),
                )
                .await
                .unwrap_err();
            assert_eq!(
                crate::canister_rejection(&failure).reject_code,
                RejectCode::CanisterError
            );
        } else if max_reply_bytes == 1 {
            assert_eq!(
                dispatch(&f, client, input).await,
                Err(ConsumerFailure::Transport)
            );
        } else {
            assert_eq!(
                dispatch(&f, client, input).await.unwrap(),
                inspect(&f, client).await
            );
        }
        assert_eq!(saved(&f, client).await, Some(command));
        let observed = inspect(&f, client).await;
        assert_eq!(
            observed,
            TenantEnrollmentResponse {
                scope: command.scope,
                enrollment: Some(TenantEnrollment {
                    generation: 1,
                    active: true
                }),
                fenced: false
            }
        );
        assert_eq!(
            dispatch(&f, client, input).await,
            Err(ConsumerFailure::Pending)
        );
        assert_eq!(
            dispatch(
                &f,
                client,
                TenantDispatch {
                    command: TenantUpdateRequest {
                        active: false,
                        ..command
                    },
                    ..input
                }
            )
            .await,
            Err(ConsumerFailure::Conflict)
        );
        assert_eq!(inspect(&f, client).await, observed);

        f.harness
            .pic
            .upgrade_canister(
                client,
                consumer_wasm(),
                blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
                Some(f.controller),
            )
            .await
            .unwrap();
        assert_eq!(saved(&f, client).await, Some(command));
        assert_eq!(
            dispatch(&f, client, input).await,
            Err(ConsumerFailure::Fenced)
        );
        assert_eq!(inspect(&f, client).await, observed);
        f.harness
            .pic
            .upgrade_canister(
                f.service,
                Fixture::wasm(),
                Fixture::installation("fixture_post_upgrade", f.operator),
                Some(f.controller),
            )
            .await
            .unwrap();
        assert_eq!(
            inspect(&f, client).await,
            TenantEnrollmentResponse {
                fenced: true,
                ..observed
            }
        );
        assert_eq!(saved(&f, client).await, Some(command));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tenant_client_observation_does_not_grant_operator_or_query_dispatch_authority() {
    let (f, client) = fixture(false).await;
    let observed = inspect(&f, client).await;
    assert_eq!(
        observed.enrollment,
        Some(TenantEnrollment {
            generation: 1,
            active: true
        })
    );
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let query: Result<TenantEnrollmentResponse, ConsumerFailure> = f
        .harness
        .pic
        .query_control_as(
            client,
            f.controller,
            "fixture_query_tenant",
            f.tenant_scope(),
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(query, Err(ConsumerFailure::Transport));
    let command = TenantUpdateRequest {
        scope: f.tenant_scope(),
        expected: observed.enrollment,
        active: false,
    };
    assert_eq!(
        dispatch(
            &f,
            client,
            TenantDispatch {
                command,
                max_reply_bytes: 4096,
                trap_after_reply: false
            }
        )
        .await,
        Err(ConsumerFailure::Transport)
    );
    assert_eq!(saved(&f, client).await, Some(command));
    assert_eq!(inspect(&f, client).await, observed);
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    let foreign: Result<TenantEnrollmentResponse, ConsumerFailure> = f
        .harness
        .pic
        .update_control_as(
            client,
            f.controller,
            "fixture_inspect_tenant",
            TenantScope {
                tenant: f.other.into(),
                ..f.tenant_scope()
            },
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(foreign, Err(ConsumerFailure::Transport));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tenant_client_retains_stale_command_without_overwriting_newer_enrollment() {
    for reactivate in [false, true] {
        let (f, client) = fixture(true).await;
        // Seed competing operator changes through the service boundary. The
        // command under test dispatches from the actual operator canister below.
        let original = f.enroll(None, true).await.unwrap();
        let command = TenantUpdateRequest {
            scope: f.tenant_scope(),
            expected: Some(original),
            active: false,
        };
        let suspended = f.enroll(Some(original), false).await.unwrap();
        let current = if reactivate {
            f.enroll(Some(suspended), true).await.unwrap()
        } else {
            suspended
        };
        // Reject both a stale active flag within one generation and an old
        // generation whose active flag once again matches the saved observation.
        let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
        assert_eq!(
            f.update_enrollment(f.operator, command).await,
            Err(TenantFailure::Conflict)
        );
        let input = TenantDispatch {
            command,
            max_reply_bytes: 4096,
            trap_after_reply: false,
        };
        assert_eq!(
            dispatch(&f, client, input).await,
            Err(ConsumerFailure::Transport)
        );
        assert_eq!(saved(&f, client).await, Some(command));
        assert_eq!(inspect(&f, client).await.enrollment, Some(current));
        assert_eq!(
            dispatch(&f, client, input).await,
            Err(ConsumerFailure::Pending)
        );
        assert_eq!(
            dispatch(
                &f,
                client,
                TenantDispatch {
                    command: TenantUpdateRequest {
                        expected: Some(current),
                        ..command
                    },
                    ..input
                }
            )
            .await,
            Err(ConsumerFailure::Conflict)
        );
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            before
        );

        f.harness
            .pic
            .upgrade_canister(
                client,
                consumer_wasm(),
                blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
                Some(f.controller),
            )
            .await
            .unwrap();
        assert_eq!(saved(&f, client).await, Some(command));
        assert_eq!(inspect(&f, client).await.enrollment, Some(current));
        assert_eq!(
            dispatch(&f, client, input).await,
            Err(ConsumerFailure::Fenced)
        );
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            before
        );
    }
}
