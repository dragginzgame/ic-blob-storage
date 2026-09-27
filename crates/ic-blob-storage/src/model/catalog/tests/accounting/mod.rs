//! Reconstruct every aggregate field from individual public journal observations.
use super::*;

fn audit(catalog: &BlobCatalog, inputs: &[ConfirmedObject]) {
    for tenant in [None, Some(p(2)), Some(p(3)), Some(p(4)), Some(p(9))] {
        let mut expected = CatalogUsage::default();
        for input in inputs
            .iter()
            .filter(|i| tenant.is_none_or(|p| p == i.first.object().tenant()))
        {
            let journal = catalog.get(input.root).unwrap();
            let lifecycle = journal.lifecycle();
            expected.objects += 1;
            expected.live_objects += usize::from(lifecycle.phase() == LifecyclePhase::Live);
            expected.pending_deletions +=
                usize::from(lifecycle.phase() == LifecyclePhase::DeletionPending);
            expected.unsettled_objects += usize::from(lifecycle.has_unsettled_obligations());
            expected.logical_bytes += u128::from(lifecycle.logical_bytes());
            expected.physical_bytes += u128::from(lifecycle.physical_bytes());
            expected.liability_bytes += u128::from(lifecycle.liability_bytes());
            expected.active_references += lifecycle.active_references() as u128;
            expected.reference_slots += lifecycle.reference_slots() as u128;
            expected.receipt_slots += journal.receipt_count() as u128;
        }
        assert_eq!(
            tenant.map_or_else(|| catalog.usage(), |p| catalog.tenant_usage(p)),
            expected
        );
    }
}

fn apply(
    catalog: &mut BlobCatalog,
    inputs: &[ConfirmedObject],
    input: ConfirmedObject,
    request: ReferenceRequest,
    result: Result<LifecycleChange, LifecycleError>,
) {
    let actor = input.first.object().tenant();
    assert_eq!(
        catalog.apply_reference(input.root, actor, request),
        Ok(ReferenceRequestOutcome::Recorded { result })
    );
    audit(catalog, inputs);
    assert_eq!(
        catalog.apply_reference(input.root, actor, request),
        Ok(ReferenceRequestOutcome::Replayed { result })
    );
    audit(catalog, inputs);
}

fn exercise(
    catalog: &mut BlobCatalog,
    inputs: &[ConfirmedObject],
    input: ConfirmedObject,
    stage: usize,
) {
    let object = input.first.object();
    let second = ReferenceKey::new(object, ReferenceId::new(n(2)));
    let missing = ReferenceKey::new(object, ReferenceId::new(n(3)));
    apply(
        catalog,
        inputs,
        input,
        request(1, ReferenceOperation::Release(missing)),
        Err(LifecycleError::UnknownReference),
    );
    apply(
        catalog,
        inputs,
        input,
        request(2, ReferenceOperation::Retain(second)),
        Ok(LifecycleChange::Changed),
    );
    assert_eq!(
        catalog.confirm_provider_deleted(input.root, object),
        Err(CatalogError::Lifecycle(
            LifecycleError::LiveReferencesRemain
        ))
    );
    assert_eq!(
        catalog.confirm_billing_stopped(input.root, object),
        Err(CatalogError::Lifecycle(
            LifecycleError::DeletionNotConfirmed
        ))
    );
    assert_eq!(
        catalog.apply_reference(
            input.root,
            object.tenant(),
            request(2, ReferenceOperation::Release(second))
        ),
        Err(CatalogError::Request(
            ReferenceRequestError::RequestConflict
        ))
    );
    audit(catalog, inputs);
    if stage == 0 {
        return;
    }
    apply(
        catalog,
        inputs,
        input,
        request(3, ReferenceOperation::Release(input.first)),
        Ok(LifecycleChange::Changed),
    );
    if stage == 1 {
        return;
    }
    apply(
        catalog,
        inputs,
        input,
        request(4, ReferenceOperation::Release(second)),
        Ok(LifecycleChange::Changed),
    );
    if stage == 2 {
        return;
    }
    for expected in [LifecycleChange::Changed, LifecycleChange::Unchanged] {
        assert_eq!(
            catalog.confirm_provider_deleted(input.root, object),
            Ok(expected)
        );
        audit(catalog, inputs);
    }
    if stage == 3 {
        return;
    }
    for expected in [LifecycleChange::Changed, LifecycleChange::Unchanged] {
        assert_eq!(
            catalog.confirm_billing_stopped(input.root, object),
            Ok(expected)
        );
        audit(catalog, inputs);
    }
}

#[test]
fn all_usage_fields_match_journals_through_mixed_lifetimes_and_receipt_failures() {
    let mut catalog = BlobCatalog::new(
        p(1),
        CatalogLimits {
            max_objects: bound(64),
            max_tenant_objects: bound(64),
            max_physical_bytes: n(u128::MAX),
            max_liability_bytes: n(u128::MAX),
            max_tenant_logical_bytes: n(u128::MAX),
            ..limits()
        },
    )
    .unwrap();
    let mut inputs = Vec::new();
    for id in 1..=64u8 {
        let input = input(
            id,
            2 + id % 3,
            u128::from(1 + id % 4),
            if id % 7 == 0 {
                0
            } else {
                u64::MAX - u64::from(id)
            },
        );
        assert_eq!(
            catalog.insert_confirmed(input),
            Ok(CatalogInsertOutcome::Inserted)
        );
        inputs.push(input);
        audit(&catalog, &inputs);
    }
    for (index, input) in inputs.iter().copied().enumerate() {
        exercise(&mut catalog, &inputs, input, index % 5);
        assert_eq!(
            catalog.insert_confirmed(input),
            Ok(CatalogInsertOutcome::Existing)
        );
        assert_eq!(
            catalog.insert_confirmed(ConfirmedObject {
                bytes: input.bytes + 1,
                ..input
            }),
            Err(CatalogError::RegistrationConflict)
        );
        audit(&catalog, &inputs);
    }
    assert_eq!(
        catalog.insert_confirmed(input(65, 2, 1, 0)),
        Err(CatalogError::Capacity(CatalogCapacity::Objects))
    );
    audit(&catalog, &inputs);
    assert!(catalog.usage().physical_bytes > u128::from(u64::MAX));
}
