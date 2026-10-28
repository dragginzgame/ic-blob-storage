use super::*;
use std::time::Duration;

fn reply_hold(f: &Fixture, hold: Option<bool>) -> u64 {
    f.harness
        .pic
        .update_control_as::<Result<u64, Failure>, _>(
            f.service,
            f.operator,
            "fixture_tenant_reply_hold",
            hold,
            REPLY_BYTES,
        )
        .unwrap()
        .unwrap()
}

#[test]
fn bounded_timeout_keeps_committed_remote_effect_and_original_intent_without_resend() {
    let (f, client) = fixture(true);
    let input = TenantDispatch {
        command: TenantUpdateRequest {
            scope: f.tenant_scope(),
            expected: None,
            active: true,
        },
        max_reply_bytes: 4096,
        trap_after_reply: false,
    };
    assert_eq!(reply_hold(&f, Some(true)), 0);
    let denied: Result<u64, Failure> = f
        .harness
        .pic
        .update_control_as(
            f.service,
            f.other,
            "fixture_tenant_reply_hold",
            Some(false),
            REPLY_BYTES,
        )
        .unwrap();
    assert_eq!(denied, Err(Failure::Denied));
    let call = f
        .harness
        .pic
        .submit_call(
            client,
            f.controller,
            "fixture_update_tenant",
            ic_blob_storage_contracts::document::encode(input).unwrap(),
        )
        .unwrap();
    for _ in 0..20 {
        f.harness.pic.tick();
        if reply_hold(&f, None) == 1 {
            break;
        }
    }
    assert_eq!(reply_hold(&f, None), 1, "mutation reached its reply hold");
    let observed = inspect(&f, client);
    assert_eq!(
        observed.enrollment,
        Some(TenantEnrollment {
            generation: 1,
            active: true,
        })
    );
    let committed = f.harness.pic.get_stable_memory(f.service);
    f.harness.pic.advance_time(Duration::from_secs(31));
    let outcome: Result<TenantEnrollmentResponse, ConsumerFailure> =
        ic_blob_storage_contracts::document::decode(
            &f.harness.pic.await_call(call).unwrap(),
            REPLY_BYTES,
        )
        .unwrap();
    assert_eq!(outcome, Err(ConsumerFailure::Rejected(6)));
    assert_eq!(saved(&f, client), Some(input.command));
    assert_eq!(dispatch(&f, client, input), Err(ConsumerFailure::Pending));
    assert_eq!(reply_hold(&f, None), 1, "uncertainty cannot resend");

    // Deliver the original late reply, then exercise the same client executor.
    assert_eq!(reply_hold(&f, Some(false)), 1);
    for _ in 0..4 {
        f.harness.pic.tick();
    }
    assert_eq!(inspect(&f, client), observed);
    assert_eq!(saved(&f, client), Some(input.command));
    assert_eq!(dispatch(&f, client, input), Err(ConsumerFailure::Pending));
    assert_eq!(reply_hold(&f, None), 1);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), committed);
}
