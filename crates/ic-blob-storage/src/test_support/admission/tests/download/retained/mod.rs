use super::*;

fn reference(input: UploadPermission, n: u128) -> ReferenceKey {
    ReferenceKey::new(input.request.object.first.object(), ReferenceId::new(id(n)))
}

fn apply(owner: &mut UploadAdmissions, input: UploadPermission, n: u128, op: ReferenceOperation) {
    owner
        .apply_reference(
            context(4),
            input.request.object.root,
            ReferenceRequest {
                id: ReferenceRequestId::new(id(n)),
                operation: op,
            },
        )
        .unwrap();
}

#[test]
fn retained_descriptor_requires_completion_and_the_exact_reference_through_cleanup() {
    let mut owner = admissions();
    let input = permission(1);
    let root = input.request.object.root;
    let first = reference(input, 1);
    let second = reference(input, 2);
    assert_eq!(
        owner.retained_content_descriptor(context(4), root, first),
        Ok(None)
    );
    owner.admit(context(4), input, 1).unwrap();
    assert_eq!(
        owner.retained_content_descriptor(context(4), root, first),
        Ok(None)
    );
    prepare(&mut owner, &input);
    assert_eq!(
        owner.retained_content_descriptor(context(4), root, first),
        Ok(None)
    );
    owner.expose(context(5), input.request, 2).unwrap();
    assert_eq!(
        owner.retained_content_descriptor(context(4), root, first),
        Ok(None)
    );
    owner.confirm_upload(input.request).unwrap();
    assert_eq!(
        owner.retained_content_descriptor(context(4), root, second),
        Ok(None)
    );
    apply(&mut owner, input, 1, ReferenceOperation::Retain(second));
    apply(&mut owner, input, 2, ReferenceOperation::Release(first));
    // The object stays live, but its original consumer has released it.
    assert_eq!(
        owner.retained_content_descriptor(context(4), root, first),
        Ok(None)
    );
    let usage = owner.catalog().usage();
    let capacity = owner.reference_capacity(context(4), query(input)).unwrap();
    let view = owner
        .retained_content_descriptor(context(4), root, second)
        .unwrap()
        .unwrap();
    assert_eq!(view.reference, second);
    assert_eq!(
        view.descriptor,
        owner
            .content_descriptor(context(4), query(input))
            .unwrap()
            .unwrap()
    );
    let copied = view.descriptor.content;
    set_active(&mut owner, false);
    assert!(
        owner
            .retained_content_descriptor(context(4), root, second)
            .unwrap()
            .is_some()
    );
    assert_eq!(owner.catalog().usage(), usage);
    assert_eq!(
        owner.reference_capacity(context(4), query(input)).unwrap(),
        capacity
    );
    apply(&mut owner, input, 3, ReferenceOperation::Release(second));
    // A previously copied successful observation cannot prevent or undo release.
    assert_eq!(
        copied.state,
        UploadRootState::Confirmed(LifecyclePhase::Live)
    );
    assert_eq!(
        owner.retained_content_descriptor(context(4), root, second),
        Ok(None)
    );
    owner
        .confirm_provider_deleted(root, first.object())
        .unwrap();
    assert_eq!(
        owner.retained_content_descriptor(context(4), root, second),
        Ok(None)
    );
    owner.confirm_billing_stopped(root, first.object()).unwrap();
    assert_eq!(
        owner.retained_content_descriptor(context(4), root, second),
        Ok(None)
    );
    assert!(
        owner
            .content_descriptor(context(4), query(input))
            .unwrap()
            .is_some()
    );
}

#[test]
fn retained_descriptor_checks_actor_and_every_object_binding_before_disclosure() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 1).unwrap();
    prepare(&mut owner, &input);
    owner.expose(context(5), input.request, 2).unwrap();
    owner.confirm_upload(input.request).unwrap();
    let root = input.request.object.root;
    let first = reference(input, 1);
    for actor in [2, 3, 5, 6] {
        assert_eq!(
            owner.retained_content_descriptor(context(actor), root, first),
            Err(UploadAdmissionError::NotProject)
        );
    }
    assert_eq!(
        owner.retained_content_descriptor(
            UploadContext {
                service: p(9),
                ..context(4)
            },
            root,
            first
        ),
        Err(UploadAdmissionError::WrongService)
    );
    let object = first.object();
    for (service, tenant, identity, expected) in [
        (
            p(9),
            p(4),
            object.identity(),
            Err(UploadAdmissionError::WrongService),
        ),
        (
            p(1),
            p(6),
            object.identity(),
            Err(UploadAdmissionError::NotProject),
        ),
        (
            p(1),
            p(4),
            ObjectIdentity {
                namespace: id(9),
                ..object.identity()
            },
            Err(UploadAdmissionError::WrongNamespace),
        ),
        (
            p(1),
            p(4),
            ObjectIdentity {
                object: id(9),
                ..object.identity()
            },
            Ok(None),
        ),
        (
            p(1),
            p(4),
            ObjectIdentity {
                incarnation: id(9),
                ..object.identity()
            },
            Ok(None),
        ),
    ] {
        let key = ReferenceKey::new(
            ObjectBinding::new(service, tenant, identity).unwrap(),
            first.reference(),
        );
        assert_eq!(
            owner.retained_content_descriptor(context(4), root, key),
            expected
        );
    }
    let foreign = ReferenceKey::new(
        ObjectBinding::new(p(1), p(6), object.identity()).unwrap(),
        first.reference(),
    );
    assert_eq!(
        owner.retained_content_descriptor(context(6), root, foreign),
        Ok(None)
    );
    assert_eq!(
        owner.retained_content_descriptor(context(4), hashes(2).provider_root, first),
        Ok(None)
    );
}

#[test]
fn cancelled_metadata_does_not_make_a_retained_descriptor() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 1).unwrap();
    prepare(&mut owner, &input);
    owner.revoke(context(4), input.request).unwrap();
    assert!(
        owner
            .content_descriptor(context(4), query(input))
            .unwrap()
            .is_some()
    );
    assert_eq!(
        owner.retained_content_descriptor(
            context(4),
            input.request.object.root,
            reference(input, 1)
        ),
        Ok(None)
    );
}
