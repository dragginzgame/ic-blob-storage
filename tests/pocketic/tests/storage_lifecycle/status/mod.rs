//! The shared query distinguishes current reference state from retained mutation receipts.
use super::*;
use blob_test_protocol::storage::reference::{ReferenceProbeFailure, ReferenceStatusClientInput};
use ic_blob_storage::dto::reference::status::{ReferenceStatusRequest, ReferenceStatusResponse};

fn request(input: ReferenceInput) -> ReferenceStatusRequest {
    let command = receipt_request(input);
    ReferenceStatusRequest {
        upload: command.upload,
        reference: command.reference,
    }
}
fn inspect(
    f: &Fixture,
    actor: Principal,
    request: ReferenceStatusRequest,
) -> Result<ReferenceStatusResponse, ReferenceFailure> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "blob_reference_status", (request,))
        .unwrap()
}
#[test]
fn reference_status_keeps_history_distinct_from_live_references_through_settlement() {
    let f = Fixture::new();
    let permission = f.exposed();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let retain = reference(permission.request, 1, u128::MAX, true);
    f.reference(retain).unwrap();
    let expected = f.receipt(retain).unwrap();
    let query = request(retain);
    let active = inspect(&f, f.tenant, query).unwrap();
    assert_eq!(
        active,
        ReferenceStatusResponse {
            request: query,
            live: true,
            fenced: false
        }
    );
    f.enroll(Some(f.tenant().unwrap()), false).unwrap();
    assert_eq!(inspect(&f, f.tenant, query), Ok(active));
    let release = reference(permission.request, 2, u128::MAX, false);
    f.reference(release).unwrap();
    assert!(!inspect(&f, f.tenant, query).unwrap().live);
    assert_eq!(f.receipt(retain), Ok(expected));
    assert!(
        inspect(
            &f,
            f.tenant,
            request(reference(permission.request, 3, 1, false))
        )
        .unwrap()
        .live
    );
    let unknown = ReferenceStatusRequest {
        reference: u128::MAX - 1,
        ..query
    };
    assert!(!inspect(&f, f.tenant, unknown).unwrap().live);
    for actor in [f.controller, f.operator, f.uploader, Principal::anonymous()] {
        assert_eq!(inspect(&f, actor, query), Err(ReferenceFailure::Denied));
    }
    f.reference(reference(permission.request, 3, 1, false))
        .unwrap();
    f.fact(permission.request, ProviderFact::Deleted).unwrap();
    f.fact(permission.request, ProviderFact::Settled).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let retired = ReferenceStatusResponse {
        live: false,
        ..active
    };
    assert_eq!(inspect(&f, f.tenant, query), Ok(retired));
    let replicated: Result<ReferenceStatusResponse, ReferenceFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_reference_status", (query,))
        .unwrap();
    assert_eq!(replicated, Ok(retired));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
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
        inspect(&f, f.tenant, query),
        Ok(ReferenceStatusResponse {
            fenced: true,
            ..retired
        })
    );
    assert_eq!(f.receipt(retain), Ok(expected));
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One actual canister client follows reply refusals and both restores"
)]
fn reference_status_client_authenticates_exact_replies_and_preserves_live_restore_fences() {
    let mut f = Fixture::new();
    let client = f.harness.pic.create_canister();
    f.harness.pic.install_canister(
        client,
        Fixture::wasm(),
        candid::encode_one(f.operator).unwrap(),
        None,
    );
    f.tenant = client;
    let permission = f.exposed();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let input = ReferenceStatusClientInput {
        request: request(reference(permission.request, 1, 1, false)),
        tenant: client,
        max_reply_bytes: 4096,
    };
    let fetch = |input: ReferenceStatusClientInput| -> Result<ReferenceStatusResponse, ReferenceProbeFailure> {
        f.harness.pic.update_candid_as(client, f.operator, "fixture_fetch_reference_status", (input,)).unwrap()
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    let client_before = f.harness.pic.get_stable_memory(client);
    let expected = ReferenceStatusResponse {
        request: input.request,
        live: true,
        fenced: false,
    };
    assert_eq!(fetch(input), Ok(expected));
    assert_eq!(
        fetch(ReferenceStatusClientInput {
            max_reply_bytes: 1,
            ..input
        }),
        Err(ReferenceProbeFailure::Limit)
    );
    assert_eq!(
        fetch(ReferenceStatusClientInput {
            tenant: f.operator,
            ..input
        }),
        Err(ReferenceProbeFailure::Binding)
    );
    assert_eq!(
        fetch(ReferenceStatusClientInput {
            request: ReferenceStatusRequest {
                upload: ReferenceUpload {
                    bytes: 11,
                    ..input.request.upload
                },
                ..input.request
            },
            ..input
        }),
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Conflict))
    );
    assert_eq!(
        fetch(ReferenceStatusClientInput {
            request: ReferenceStatusRequest {
                reference: 0,
                ..input.request
            },
            ..input
        }),
        Err(ReferenceProbeFailure::Invalid)
    );
    let query: Result<ReferenceStatusResponse, ReferenceProbeFailure> = f
        .harness
        .pic
        .query_candid_as(
            client,
            f.operator,
            "fixture_nonreplicated_reference_status",
            (input,),
        )
        .unwrap();
    assert_eq!(query, Err(ReferenceProbeFailure::Execution));
    let denied: Result<ReferenceStatusResponse, ReferenceProbeFailure> = f
        .harness
        .pic
        .update_candid_as(client, f.other, "fixture_fetch_reference_status", (input,))
        .unwrap();
    assert_eq!(denied, Err(ReferenceProbeFailure::Denied));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(f.harness.pic.get_stable_memory(client), client_before);
    f.enroll(Some(f.tenant().unwrap()), false).unwrap();
    assert_eq!(fetch(input), Ok(expected));
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    let restored = ReferenceStatusResponse {
        fenced: true,
        ..expected
    };
    assert_eq!(fetch(input), Ok(restored));
    f.harness
        .pic
        .upgrade_canister(
            client,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            None,
        )
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let client_before = f.harness.pic.get_stable_memory(client);
    assert_eq!(fetch(input), Ok(restored));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(f.harness.pic.get_stable_memory(client), client_before);
}
