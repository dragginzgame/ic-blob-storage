use super::*;
use std::time::Duration;

async fn reply_hold(f: &Fixture, hold: Option<bool>) -> u64 {
    f.harness
        .pic
        .update_control_as::<Result<u64, Failure>, _>(
            f.service,
            f.operator,
            "fixture_tenant_reply_hold",
            hold,
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn bounded_timeout_keeps_committed_remote_effect_and_original_intent_without_resend() {
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
    assert_eq!(reply_hold(&f, Some(true)).await, 0);
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
        .await
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
        .await
        .unwrap();
    for _ in 0..20 {
        f.harness.pic.tick().await.unwrap();
        if reply_hold(&f, None).await == 1 {
            break;
        }
    }
    assert_eq!(
        reply_hold(&f, None).await,
        1,
        "mutation reached its reply hold"
    );
    let observed = inspect(&f, client).await;
    assert_eq!(
        observed.enrollment,
        Some(TenantEnrollment {
            generation: 1,
            active: true,
        })
    );
    let committed = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    f.harness
        .pic
        .advance_time(Duration::from_secs(31))
        .await
        .unwrap();
    let outcome: Result<TenantEnrollmentResponse, ConsumerFailure> =
        ic_blob_storage_contracts::document::decode(
            &f.harness.pic.await_call(call).await.unwrap(),
            REPLY_BYTES,
        )
        .unwrap();
    assert_eq!(outcome, Err(ConsumerFailure::Rejected(6)));
    assert_eq!(saved(&f, client).await, Some(input.command));
    assert_eq!(
        dispatch(&f, client, input).await,
        Err(ConsumerFailure::Pending)
    );
    assert_eq!(reply_hold(&f, None).await, 1, "uncertainty cannot resend");

    // Deliver the original late reply, then exercise the same client executor.
    assert_eq!(reply_hold(&f, Some(false)).await, 1);
    for _ in 0..4 {
        f.harness.pic.tick().await.unwrap();
    }
    assert_eq!(inspect(&f, client).await, observed);
    assert_eq!(saved(&f, client).await, Some(input.command));
    assert_eq!(
        dispatch(&f, client, input).await,
        Err(ConsumerFailure::Pending)
    );
    assert_eq!(reply_hold(&f, None).await, 1);
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        committed
    );
}
