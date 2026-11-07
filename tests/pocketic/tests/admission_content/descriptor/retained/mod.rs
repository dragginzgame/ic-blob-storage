use super::*;
use blob_test_protocol::admission::input::{RetainedDescriptor, RetainedDescriptorInput};

async fn lifecycle(
    f: &Fixture,
    actor: Principal,
    command: LifecycleCommand,
) -> Result<Outcome, Failure> {
    f.call(actor, Command::FixtureLifecycle(command)).await
}

async fn inspect(
    f: &Fixture,
    actor: Principal,
    input: RetainedDescriptorInput,
) -> Result<Option<RetainedDescriptor>, blob_test_protocol::storage::Failure> {
    ic_blob_storage_pocketic_tests::control_calls::ControlCallExt::query_control_as(
        &f.harness.pic,
        f.service,
        actor,
        "retained_content_descriptor",
        (input,),
        blob_test_protocol::fixture_wire::MAX_BYTES,
    )
    .await
    .map(|(reply,)| reply)
    .unwrap()
}

fn reference(p: Permission, reference: u128, operation: u128, retain: bool) -> LifecycleCommand {
    LifecycleCommand::Reference {
        object: p.request,
        reference,
        operation,
        retain,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn retained_descriptor_requires_exact_live_reference_despite_old_successful_receipts() {
    let f = Fixture::new().await;
    let enrollment = f.enroll().await;
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v).await;
    let first = RetainedDescriptorInput {
        content: lookup(&f, p),
        object: p.request.id,
        incarnation: 1,
        reference: 1,
    };
    let second = RetainedDescriptorInput {
        reference: 2,
        ..first
    };
    assert_eq!(inspect(&f, f.project, first).await, Ok(None));
    f.admit(p).await;
    assert_eq!(inspect(&f, f.project, first).await, Ok(None));
    f.prepare(p, &v).await;
    assert_eq!(inspect(&f, f.project, first).await, Ok(None));
    f.call(f.uploader, Command::Expose(p.request.root))
        .await
        .unwrap();
    let descriptor = inspect(&f, f.project, first).await.unwrap().unwrap();
    assert_eq!(descriptor.reference, first);
    assert_eq!(descriptor.descriptor.content.state, ContentState::Live);
    check_body(&descriptor.descriptor, b"abc");
    check_authority(&f, first).await;
    assert_eq!(inspect(&f, f.project, second).await, Ok(None));
    let retain = reference(p, 2, 1, true);
    lifecycle(&f, f.project, retain).await.unwrap();
    lifecycle(&f, f.project, reference(p, 1, 2, false))
        .await
        .unwrap();
    let before = f.observe(p).await;
    assert_eq!(inspect(&f, f.project, first).await, Ok(None));
    let still_live = inspect(&f, f.project, second).await.unwrap().unwrap();
    assert_eq!(still_live.reference, second);
    assert_eq!(f.observe(p).await, before);
    f.call(f.operator, f.enrollment(Some(enrollment), false))
        .await
        .unwrap();
    f.restart().await;
    assert_eq!(inspect(&f, f.project, second).await, Ok(Some(still_live)));
    lifecycle(&f, f.project, reference(p, 2, 3, false))
        .await
        .unwrap();
    // Retrying a formerly successful retain recovers its receipt, not liveness.
    assert_eq!(
        lifecycle(&f, f.project, retain).await,
        Ok(Outcome::Reference {
            replayed: true,
            result: Ok(true)
        })
    );
    assert_eq!(inspect(&f, f.project, second).await, Ok(None));
    for command in [
        LifecycleCommand::SubstituteDeletion(p.request),
        LifecycleCommand::SubstituteSettlement(p.request),
    ] {
        lifecycle(&f, f.operator, command).await.unwrap();
        assert_eq!(inspect(&f, f.project, second).await, Ok(None));
    }
    f.restart().await;
    assert_eq!(inspect(&f, f.project, second).await, Ok(None));
    // Historical descriptors remain available for recovery/accounting inspection.
    assert_eq!(
        read(&f, f.project, first.content)
            .await
            .unwrap()
            .unwrap()
            .content
            .state,
        ContentState::Settled
    );
}

#[expect(
    clippy::too_many_lines,
    reason = "Keep malformed, authority, scope and absent-history checks together against the same retained descriptor"
)]
async fn check_authority(f: &Fixture, input: RetainedDescriptorInput) {
    let oversized = f
        .harness
        .pic
        .query_call(
            f.service,
            f.project,
            "retained_content_descriptor",
            vec![0; 4097],
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&oversized).reject_code,
        RejectCode::CanisterError
    );
    for actor in [
        f.operator,
        f.controller,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(
            inspect(f, actor, input).await,
            Err(blob_test_protocol::storage::Failure::Denied)
        );
    }
    for bad in [
        RetainedDescriptorInput { object: 0, ..input },
        RetainedDescriptorInput {
            incarnation: 0,
            ..input
        },
        RetainedDescriptorInput {
            reference: 0,
            ..input
        },
    ] {
        assert_eq!(
            inspect(f, f.project, bad).await,
            Err(blob_test_protocol::storage::Failure::Invalid)
        );
    }
    for bad in [
        RetainedDescriptorInput { object: 2, ..input },
        RetainedDescriptorInput {
            incarnation: 2,
            ..input
        },
        RetainedDescriptorInput {
            reference: 2,
            ..input
        },
        RetainedDescriptorInput {
            content: ContentLookup {
                root: [0; 32],
                ..input.content
            },
            ..input
        },
    ] {
        assert_eq!(inspect(f, f.project, bad).await, Ok(None));
    }
    assert_eq!(
        inspect(
            f,
            f.project,
            RetainedDescriptorInput {
                content: ContentLookup {
                    service: f.other,
                    ..input.content
                },
                ..input
            }
        )
        .await,
        Err(blob_test_protocol::storage::Failure::Binding)
    );
    assert_eq!(
        inspect(
            f,
            f.project,
            RetainedDescriptorInput {
                content: ContentLookup {
                    namespace: 2,
                    ..input.content
                },
                ..input
            }
        )
        .await,
        Err(blob_test_protocol::storage::Failure::Binding)
    );
    assert_eq!(
        inspect(
            f,
            f.other,
            RetainedDescriptorInput {
                content: ContentLookup {
                    tenant: f.other,
                    ..input.content
                },
                ..input
            }
        )
        .await,
        Ok(None)
    );
}
