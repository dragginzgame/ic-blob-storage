//! Byte inputs for integrity/readback experiments, separate from admission metadata.
use blob_test_protocol::journey::{JourneyManifest, JourneyUpload};
pub(crate) struct Vector {
    pub upload: JourneyUpload,
    pub manifest: JourneyManifest,
    pub bytes: Vec<u8>,
}
pub(crate) fn vector(name: &str, id: u8) -> Vector {
    let mut declaration = super::vectors::vector(name, id);
    let source = super::vectors::source(name);
    let bytes: Vec<u8> = (0..declaration.upload.bytes)
        .map(|i| match source["pattern"].as_str().expect("pattern") {
            "abc" => b"abc"[usize::try_from(i).expect("index")],
            "linear_mod251" => u8::try_from((i * 31 + 7) % 251).expect("pattern byte"),
            "zeroes" => 0,
            _ => panic!("unsupported fixture pattern"),
        })
        .collect();
    if !declaration
        .manifest
        .headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("Content-Length"))
    {
        use ic_blob_storage_contracts::identity::caffeine::{
            CaffeineHashLimits, CaffeineHeader, manifest::builder::CaffeineManifestBuilder,
        };
        declaration
            .manifest
            .headers
            .push(("Content-Length".into(), bytes.len().to_string()));
        let headers: Vec<_> = declaration
            .manifest
            .headers
            .iter()
            .map(|(name, value)| CaffeineHeader { name, value })
            .collect();
        let mut builder = CaffeineManifestBuilder::new(
            declaration.upload.bytes,
            &headers,
            CaffeineHashLimits {
                max_content_bytes: declaration.upload.bytes.try_into().unwrap(),
                max_append_bytes: (64 * 1024).try_into().unwrap(),
                max_headers: 8.try_into().unwrap(),
                max_header_bytes: 4096.try_into().unwrap(),
            },
            32.try_into().unwrap(),
        )
        .unwrap();
        for frame in bytes.chunks(64 * 1024) {
            builder.append(builder.received_bytes(), frame).unwrap();
        }
        let built = builder.finish().unwrap();
        assert_eq!(
            built.hashes().content_digest.as_bytes(),
            &declaration.upload.digest
        );
        declaration.upload.root = *built.hashes().provider_root.as_bytes();
    }
    Vector {
        upload: declaration.upload,
        manifest: declaration.manifest,
        bytes,
    }
}
