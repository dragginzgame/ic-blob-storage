//! Actual canister actor, committed command intent and shared client recovery.
use super::*;
use blob_test_protocol::consumer::{Failure as ConsumerFailure, TenantDispatch};

fn consumer_wasm() -> Vec<u8> {
    std::fs::read(fixture_path("BLOB_CONSUMER_PROBE_WASM")).unwrap()
}
fn fixture(operator_client: bool) -> (Fixture, Principal) {
    let harness = Harness::new();
    let driver = Fake::principal(2);
    let client = harness.pic.create_canister_with_settings(
        Some(driver),
        Some(CanisterSettings {
            controllers: Some(vec![driver]),
            ..CanisterSettings::default()
        }),
    );
    let mut f = Fixture::with_operator(
        harness,
        if operator_client {
            client
        } else {
            Fake::principal(1)
        },
    );
    f.harness.pic.install_canister(
        client,
        consumer_wasm(),
        candid::encode_args((driver, f.service)).unwrap(),
        Some(driver),
    );
    if !operator_client {
        f.tenant = client;
        f.enroll(None, true).unwrap();
    }
    (f, client)
}
fn saved(f: &Fixture, client: Principal) -> Option<TenantUpdateRequest> {
    f.harness
        .pic
        .query_candid_as::<Result<Option<TenantUpdateRequest>, ConsumerFailure>, _>(
            client,
            f.controller,
            "fixture_tenant_command",
            (),
        )
        .unwrap()
        .unwrap()
}
fn inspect(f: &Fixture, client: Principal) -> TenantEnrollmentResponse {
    f.harness
        .pic
        .update_candid_as::<Result<TenantEnrollmentResponse, ConsumerFailure>, _>(
            client,
            f.controller,
            "fixture_inspect_tenant",
            (f.tenant_scope(),),
        )
        .unwrap()
        .unwrap()
}
fn dispatch(
    f: &Fixture,
    client: Principal,
    input: TenantDispatch,
) -> Result<TenantEnrollmentResponse, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(client, f.controller, "fixture_update_tenant", (input,))
        .unwrap()
}

#[test]
fn tenant_client_retains_command_after_success_small_reply_or_callback_trap() {
    for (max_reply_bytes, trap_after_reply) in [(4096, false), (1, false), (4096, true)] {
        let (f, client) = fixture(true);
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
        assert_eq!(saved(&f, client), None);
        let denied: Result<TenantEnrollmentResponse, ConsumerFailure> = f
            .harness
            .pic
            .update_candid_as(client, f.other, "fixture_update_tenant", (input,))
            .unwrap();
        assert_eq!(denied, Err(ConsumerFailure::Denied));
        assert_eq!(saved(&f, client), None);
        if trap_after_reply {
            let failure = f
                .harness
                .pic
                .update_call(
                    client,
                    f.controller,
                    "fixture_update_tenant",
                    candid::encode_one(input).unwrap(),
                )
                .unwrap_err();
            assert_eq!(failure.reject_code, RejectCode::CanisterError);
        } else if max_reply_bytes == 1 {
            assert_eq!(dispatch(&f, client, input), Err(ConsumerFailure::Transport));
        } else {
            assert_eq!(dispatch(&f, client, input).unwrap(), inspect(&f, client));
        }
        assert_eq!(saved(&f, client), Some(command));
        let observed = inspect(&f, client);
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
        assert_eq!(dispatch(&f, client, input), Err(ConsumerFailure::Pending));
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
            ),
            Err(ConsumerFailure::Conflict)
        );
        assert_eq!(inspect(&f, client), observed);

        f.harness
            .pic
            .upgrade_canister(
                client,
                consumer_wasm(),
                candid::encode_args(()).unwrap(),
                Some(f.controller),
            )
            .unwrap();
        assert_eq!(saved(&f, client), Some(command));
        assert_eq!(dispatch(&f, client, input), Err(ConsumerFailure::Fenced));
        assert_eq!(inspect(&f, client), observed);
        f.harness
            .pic
            .upgrade_canister(
                f.service,
                Fixture::wasm(),
                candid::encode_one(f.operator).unwrap(),
                Some(f.controller),
            )
            .unwrap();
        assert_eq!(
            inspect(&f, client),
            TenantEnrollmentResponse {
                fenced: true,
                ..observed
            }
        );
        assert_eq!(saved(&f, client), Some(command));
    }
}

#[test]
fn tenant_client_observation_does_not_grant_operator_or_query_dispatch_authority() {
    let (f, client) = fixture(false);
    let observed = inspect(&f, client);
    assert_eq!(
        observed.enrollment,
        Some(TenantEnrollment {
            generation: 1,
            active: true
        })
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    let query: Result<TenantEnrollmentResponse, ConsumerFailure> = f
        .harness
        .pic
        .query_candid_as(
            client,
            f.controller,
            "fixture_query_tenant",
            (f.tenant_scope(),),
        )
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
        ),
        Err(ConsumerFailure::Transport)
    );
    assert_eq!(saved(&f, client), Some(command));
    assert_eq!(inspect(&f, client), observed);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    let foreign: Result<TenantEnrollmentResponse, ConsumerFailure> = f
        .harness
        .pic
        .update_candid_as(
            client,
            f.controller,
            "fixture_inspect_tenant",
            (TenantScope {
                tenant: f.other,
                ..f.tenant_scope()
            },),
        )
        .unwrap();
    assert_eq!(foreign, Err(ConsumerFailure::Transport));
}

#[test]
fn tenant_client_retains_stale_command_without_overwriting_newer_enrollment() {
    for reactivate in [false, true] {
        let (f, client) = fixture(true);
        // Seed competing operator changes through the service boundary. The
        // command under test dispatches from the actual operator canister below.
        let original = f.enroll(None, true).unwrap();
        let command = TenantUpdateRequest {
            scope: f.tenant_scope(),
            expected: Some(original),
            active: false,
        };
        let suspended = f.enroll(Some(original), false).unwrap();
        let current = if reactivate {
            f.enroll(Some(suspended), true).unwrap()
        } else {
            suspended
        };
        // Reject both a stale active flag within one generation and an old
        // generation whose active flag once again matches the saved observation.
        let before = f.harness.pic.get_stable_memory(f.service);
        assert_eq!(
            f.update_enrollment(f.operator, command),
            Err(TenantFailure::Conflict)
        );
        let input = TenantDispatch {
            command,
            max_reply_bytes: 4096,
            trap_after_reply: false,
        };
        assert_eq!(dispatch(&f, client, input), Err(ConsumerFailure::Transport));
        assert_eq!(saved(&f, client), Some(command));
        assert_eq!(inspect(&f, client).enrollment, Some(current));
        assert_eq!(dispatch(&f, client, input), Err(ConsumerFailure::Pending));
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
            ),
            Err(ConsumerFailure::Conflict)
        );
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);

        f.harness
            .pic
            .upgrade_canister(
                client,
                consumer_wasm(),
                candid::encode_args(()).unwrap(),
                Some(f.controller),
            )
            .unwrap();
        assert_eq!(saved(&f, client), Some(command));
        assert_eq!(inspect(&f, client).enrollment, Some(current));
        assert_eq!(dispatch(&f, client, input), Err(ConsumerFailure::Fenced));
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    }
}
