use super::*;
mod client;
use ic_blob_storage::dto::download::{DownloadFailure, DownloadRequest, DownloadResponse};
use ic_blob_storage::model::identity::{
    ProviderRootHash, caffeine::verification::CaffeineRootVerifier,
};
impl Fixture {
    fn download(
        &self,
        actor: Principal,
        input: RetainedDescriptorInput,
    ) -> Result<DownloadResponse, DownloadFailure> {
        self.harness
            .pic
            .update_candid_as(
                self.service,
                actor,
                "blob_download_descriptor",
                (wire(input),),
            )
            .unwrap()
    }
    fn download_input(&self) -> (Permission, PreparationInput, RetainedDescriptorInput) {
        self.enroll(None, true).unwrap();
        let (permission, preparation) = self.permission(1, 1);
        self.admit(self.tenant, permission).unwrap();
        self.prepare(&preparation).unwrap();
        let input = RetainedDescriptorInput {
            content: self.content_input(permission.request),
            object: 1,
            incarnation: 1,
            reference: 1,
        };
        (permission, preparation, input)
    }
}
#[test]
fn operational_download_descriptor_preserves_owner_and_hash_inputs_without_a_body_call() {
    let f = Fixture::new();
    let (permission, preparation, input) = f.download_input();
    assert_eq!(
        f.download(f.tenant, input),
        Err(DownloadFailure::Unavailable)
    );
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let view = f.download(f.tenant, input).unwrap();
    assert_eq!(view.owner, f.service);
    assert_ne!(view.owner, f.operator);
    assert_ne!(view.owner, f.tenant);
    assert_eq!(view.project, "fixture project/β?&=");
    assert_eq!(view.request, wire(input));
    assert_eq!(view.bytes, permission.request.bytes);
    assert_eq!(headers(&view), preparation.manifest.headers);
    let root = ProviderRootHash::try_from(permission.request.root.as_slice()).unwrap();
    // Actual update-delivered metadata feeds the existing off-canister verifier.
    // No live gateway is contacted and a successful prefix is never disclosed.
    let headers = view
        .headers
        .iter()
        .map(|h| CaffeineHeader {
            name: &h.name,
            value: &h.value,
        })
        .collect::<Vec<_>>();
    let mut verifier = CaffeineRootVerifier::new(
        root,
        permission.request.bytes,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(10).unwrap(),
            max_append_bytes: NonZeroUsize::new(10).unwrap(),
            max_headers: NonZeroUsize::new(8).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
        },
    )
    .unwrap();
    verifier.append(0, &[1; 4]).unwrap();
    verifier.append(4, &[1; 6]).unwrap();
    assert_eq!(verifier.finish().unwrap().provider_root, root);
    for actor in [
        f.operator,
        f.controller,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(f.download(actor, input), Err(DownloadFailure::Denied));
    }
    assert_eq!(
        f.download(
            f.tenant,
            RetainedDescriptorInput {
                incarnation: 2,
                ..input
            }
        ),
        Err(DownloadFailure::Unavailable)
    );
    let mut foreign = input;
    foreign.content.service = f.other;
    assert_eq!(f.download(f.tenant, foreign), Err(DownloadFailure::Binding));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}
#[test]
fn operational_download_rejects_released_suspended_and_restored_content() {
    let f = Fixture::new();
    let (permission, _, first) = f.download_input();
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 2,
        operation: 1,
        retain: true,
    })
    .unwrap();
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 1,
        operation: 2,
        retain: false,
    })
    .unwrap();
    let second = RetainedDescriptorInput {
        reference: 2,
        ..first
    };
    assert_eq!(
        f.download(f.tenant, first),
        Err(DownloadFailure::Unavailable)
    );
    let view = f.download(f.tenant, second).unwrap();
    f.enroll(Some(f.tenant().unwrap()), false).unwrap();
    assert_eq!(f.download(f.tenant, second), Err(DownloadFailure::Inactive));
    assert_eq!(
        f.retained(f.tenant, second)
            .unwrap()
            .unwrap()
            .descriptor
            .headers,
        headers(&view)
    );
    f.enroll(Some(f.tenant().unwrap()), true).unwrap();
    assert_eq!(f.download(f.tenant, second), Ok(view.clone()));
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(f.download(f.tenant, second), Err(DownloadFailure::Fenced));
    assert_eq!(
        f.retained(f.tenant, second)
            .unwrap()
            .unwrap()
            .descriptor
            .headers,
        headers(&view)
    );
}

fn wire(input: RetainedDescriptorInput) -> DownloadRequest {
    DownloadRequest {
        service: input.content.service,
        tenant: input.content.tenant,
        namespace: input.content.namespace,
        root: input.content.root,
        object: input.object,
        incarnation: input.incarnation,
        reference: input.reference,
    }
}
fn headers(view: &DownloadResponse) -> Vec<(String, String)> {
    view.headers
        .iter()
        .map(|h| (h.name.clone(), h.value.clone()))
        .collect()
}
