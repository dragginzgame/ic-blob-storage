//! Native client preparation, bounded IC admission and consumer verification.
use super::*;
use blob_test_protocol::admission::input::{RetainedDescriptor, RetainedDescriptorInput};
use ic_blob_storage::model::identity::caffeine::manifest::builder::CaffeineManifestBuilder;

const BYTES: usize = 10 * 1024 * 1024;

fn frames(mut accept: impl FnMut(u64, &[u8])) {
    let mut frame = vec![0; 65_537];
    for offset in (0..BYTES).step_by(frame.len()) {
        let count = frame.len().min(BYTES - offset);
        for (index, byte) in frame[..count].iter_mut().enumerate() {
            *byte = u8::try_from(((offset + index) * 31 + 7) % 251).unwrap();
        }
        accept(offset as u64, &frame[..count]);
    }
}

fn limits() -> CaffeineHashLimits {
    CaffeineHashLimits {
        max_content_bytes: NonZeroU64::new(BYTES as u64).unwrap(),
        max_append_bytes: NonZeroUsize::new(65_537).unwrap(),
        max_headers: NonZeroUsize::new(8).unwrap(),
        max_header_bytes: NonZeroUsize::new(1024).unwrap(),
    }
}

#[test]
fn client_prepared_ten_mib_manifest_enters_service_without_a_body_relay() {
    let f = Fixture::new();
    f.enroll();
    let mut vector = vectors::vector("media-10485760", 1);
    let headers: Vec<_> = vector
        .manifest
        .headers
        .iter()
        .map(|(name, value)| CaffeineHeader { name, value })
        .collect();
    let mut builder = CaffeineManifestBuilder::new(
        BYTES as u64,
        &headers,
        limits(),
        NonZeroUsize::new(10).unwrap(),
    )
    .unwrap();
    frames(|offset, frame| builder.append(offset, frame).unwrap());
    let prepared = builder.finish().unwrap();
    assert_eq!(
        prepared.hashes().provider_root.as_bytes(),
        &vector.upload.root
    );
    vector.manifest.chunks = prepared
        .manifest()
        .chunks()
        .iter()
        .map(|hash| *hash.as_bytes())
        .collect();
    let permission = f.permission(&vector);
    f.admit(permission);
    // Only the computed leaves and original metadata cross the IC boundary.
    f.prepare(permission, &vector);
    let input = RetainedDescriptorInput {
        content: lookup(&f, permission),
        object: permission.request.id,
        incarnation: 1,
        reference: 1,
    };
    let inspect = || -> Result<Option<RetainedDescriptor>, Failure> {
        f.harness
            .pic
            .query_candid_as(
                f.service,
                f.project,
                "retained_content_descriptor",
                (input,),
            )
            .unwrap()
    };
    assert_eq!(inspect(), Ok(None));
    f.call(f.uploader, Command::Expose(permission.request.root))
        .unwrap();
    assert_eq!(inspect(), Ok(None));
    f.call(
        f.operator,
        Command::FixtureLifecycle(LifecycleCommand::SubstituteCompletion(permission.request)),
    )
    .unwrap();
    let descriptor = inspect().unwrap().unwrap().descriptor;
    assert_eq!(descriptor.headers, vector.manifest.headers);
    let headers: Vec<_> = descriptor
        .headers
        .iter()
        .map(|(name, value)| CaffeineHeader { name, value })
        .collect();
    let mut verifier = CaffeineRootVerifier::new(
        descriptor
            .content
            .request
            .root
            .as_slice()
            .try_into()
            .unwrap(),
        descriptor.content.request.bytes,
        &headers,
        limits(),
    )
    .unwrap();
    frames(|offset, frame| verifier.append(offset, frame).unwrap());
    assert_eq!(verifier.finish().unwrap(), prepared.hashes());
}
