use super::*;
use blob_test_protocol::admission::input::{RetainedDescriptor, RetainedDescriptorInput};

fn lifecycle(f: &Fixture, actor: Principal, command: LifecycleCommand) -> Result<Outcome, Failure> {
    f.call(actor, Command::FixtureLifecycle(command))
}

fn inspect(
    f: &Fixture,
    actor: Principal,
    input: RetainedDescriptorInput,
) -> Result<Option<RetainedDescriptor>, Failure> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "retained_content_descriptor", (input,))
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

#[test]
fn retained_descriptor_requires_exact_live_reference_despite_old_successful_receipts() {
    let f = Fixture::new();
    let enrollment = f.enroll();
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v);
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
    assert_eq!(inspect(&f, f.project, first), Ok(None));
    f.admit(p);
    assert_eq!(inspect(&f, f.project, first), Ok(None));
    f.prepare(p, &v);
    assert_eq!(inspect(&f, f.project, first), Ok(None));
    f.call(f.uploader, Command::Expose(p.request.root)).unwrap();
    assert_eq!(inspect(&f, f.project, first), Ok(None));
    lifecycle(
        &f,
        f.operator,
        LifecycleCommand::SubstituteCompletion(p.request),
    )
    .unwrap();
    let descriptor = inspect(&f, f.project, first).unwrap().unwrap();
    assert_eq!(descriptor.reference, first);
    assert_eq!(descriptor.descriptor.content.state, ContentState::Live);
    check_body(&descriptor.descriptor, b"abc");
    check_authority(&f, first);
    assert_eq!(inspect(&f, f.project, second), Ok(None));
    let retain = reference(p, 2, 1, true);
    lifecycle(&f, f.project, retain).unwrap();
    lifecycle(&f, f.project, reference(p, 1, 2, false)).unwrap();
    let before = f.observe(p);
    assert_eq!(inspect(&f, f.project, first), Ok(None));
    let still_live = inspect(&f, f.project, second).unwrap().unwrap();
    assert_eq!(still_live.reference, second);
    assert_eq!(f.observe(p), before);
    f.call(f.operator, f.enrollment(Some(enrollment), false))
        .unwrap();
    f.restart();
    assert_eq!(inspect(&f, f.project, second), Ok(Some(still_live)));
    lifecycle(&f, f.project, reference(p, 2, 3, false)).unwrap();
    // Retrying a formerly successful retain recovers its receipt, not liveness.
    assert_eq!(
        lifecycle(&f, f.project, retain),
        Ok(Outcome::Reference {
            replayed: true,
            result: Ok(true)
        })
    );
    assert_eq!(inspect(&f, f.project, second), Ok(None));
    for command in [
        LifecycleCommand::SubstituteDeletion(p.request),
        LifecycleCommand::SubstituteSettlement(p.request),
    ] {
        lifecycle(&f, f.operator, command).unwrap();
        assert_eq!(inspect(&f, f.project, second), Ok(None));
    }
    f.restart();
    assert_eq!(inspect(&f, f.project, second), Ok(None));
    // Historical descriptors remain available for recovery/accounting inspection.
    assert_eq!(
        read(&f, f.project, first.content)
            .unwrap()
            .unwrap()
            .content
            .state,
        ContentState::Settled
    );
}

fn check_authority(f: &Fixture, input: RetainedDescriptorInput) {
    let oversized = f
        .harness
        .pic
        .query_call(
            f.service,
            f.project,
            "retained_content_descriptor",
            vec![0; 4097],
        )
        .unwrap_err();
    assert_eq!(oversized.reject_code, RejectCode::CanisterError);
    for actor in [
        f.operator,
        f.controller,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(inspect(f, actor, input), Err(Failure::NotProject));
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
        assert_eq!(inspect(f, f.project, bad), Err(Failure::InvalidInput));
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
        assert_eq!(inspect(f, f.project, bad), Ok(None));
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
        ),
        Err(Failure::WrongService)
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
        ),
        Err(Failure::WrongNamespace)
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
        ),
        Ok(None)
    );
}
