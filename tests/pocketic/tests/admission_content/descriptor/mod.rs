//! Actual caller authorization and descriptor-to-client verification.
use super::*;
use blob_test_protocol::admission::{ContentDescriptor, release::LifecycleCommand};
use ic_blob_storage::model::identity::caffeine::{
    CaffeineHashLimits, CaffeineHeader, verification::CaffeineRootVerifier,
};
use std::num::{NonZeroU64, NonZeroUsize};

mod preparation;
mod retained;

fn read(
    f: &Fixture,
    actor: Principal,
    input: ContentLookup,
) -> Result<Option<ContentDescriptor>, Failure> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "content_descriptor", (input,))
        .unwrap()
}

fn lookup(f: &Fixture, p: Permission) -> ContentLookup {
    ContentLookup {
        service: f.service,
        tenant: p.request.tenant,
        namespace: 1,
        root: p.request.root,
    }
}

fn check_body(descriptor: &ContentDescriptor, bytes: &[u8]) {
    let headers: Vec<_> = descriptor
        .headers
        .iter()
        .map(|(name, value)| CaffeineHeader { name, value })
        .collect();
    let root = descriptor
        .content
        .request
        .root
        .as_slice()
        .try_into()
        .unwrap();
    let mut verifier = CaffeineRootVerifier::new(
        root,
        descriptor.content.request.bytes,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(10 * 1024 * 1024).unwrap(),
            max_append_bytes: NonZeroUsize::new(65_536).unwrap(),
            max_headers: NonZeroUsize::new(8).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
        },
    )
    .unwrap();
    for chunk in bytes.chunks(65_536) {
        verifier.append(verifier.received_bytes(), chunk).unwrap();
    }
    assert_eq!(verifier.finish().unwrap().provider_root, root);
}

fn check_authority(f: &Fixture, input: ContentLookup) {
    let oversized = f
        .harness
        .pic
        .query_call(f.service, f.project, "content_descriptor", vec![0; 4097])
        .unwrap_err();
    assert_eq!(oversized.reject_code, RejectCode::CanisterError);
    for actor in [
        f.operator,
        f.controller,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(read(f, actor, input), Err(Failure::NotProject));
    }
    assert_eq!(
        read(
            f,
            f.other,
            ContentLookup {
                tenant: f.other,
                ..input
            }
        ),
        Ok(None)
    );
    assert_eq!(
        read(
            f,
            f.project,
            ContentLookup {
                service: f.other,
                ..input
            }
        ),
        Err(Failure::WrongService)
    );
    assert_eq!(
        read(
            f,
            f.project,
            ContentLookup {
                namespace: 2,
                ..input
            }
        ),
        Err(Failure::WrongNamespace)
    );
}

#[test]
fn descriptor_keeps_authority_original_metadata_and_current_lifecycle() {
    let f = Fixture::new();
    f.enroll();
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v);
    let input = lookup(&f, p);
    assert_eq!(read(&f, f.project, input), Ok(None));
    f.admit(p);
    assert_eq!(read(&f, f.project, input), Ok(None));
    f.prepare(p, &v);
    let first = read(&f, f.project, input).unwrap().unwrap();
    assert_eq!(first.content.request, p.request);
    assert_eq!(first.content.state, ContentState::Reserved);
    assert_eq!(first.headers, v.manifest.headers);
    let original = f.observe(p);
    check_authority(&f, input);
    let mut reordered = v.manifest.clone();
    reordered.headers.reverse();
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, reordered)),
        Ok(Outcome::Changed(false))
    );
    assert_eq!(read(&f, f.project, input), Ok(Some(first.clone())));
    assert_eq!(f.observe(p), original);
    f.call(f.uploader, Command::Expose(p.request.root)).unwrap();
    let commands = [
        (
            ContentState::ExposurePossible,
            LifecycleCommand::SubstituteCompletion(p.request),
        ),
        (
            ContentState::Live,
            LifecycleCommand::Reference {
                object: p.request,
                reference: 1,
                operation: 1,
                retain: false,
            },
        ),
        (
            ContentState::DeletionPending,
            LifecycleCommand::SubstituteDeletion(p.request),
        ),
        (
            ContentState::ProviderDeleted,
            LifecycleCommand::SubstituteSettlement(p.request),
        ),
    ];
    for (state, command) in commands {
        let view = read(&f, f.project, input).unwrap().unwrap();
        assert_eq!(view.content.state, state);
        assert_eq!(view.headers, first.headers);
        if state == ContentState::Live {
            check_body(&view, b"abc");
        }
        let actor = if matches!(command, LifecycleCommand::Reference { .. }) {
            f.project
        } else {
            f.operator
        };
        f.call(actor, Command::FixtureLifecycle(command)).unwrap();
    }
    f.restart();
    let settled = read(&f, f.project, input).unwrap().unwrap();
    assert_eq!(settled.content.state, ContentState::Settled);
    assert_eq!(settled.headers, first.headers);
}
