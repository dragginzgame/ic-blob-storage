//! Byte inputs for integrity/readback experiments, separate from admission metadata.
use blob_test_protocol::journey::{JourneyManifest, JourneyUpload};
pub(crate) struct Vector {
    pub upload: JourneyUpload,
    pub manifest: JourneyManifest,
    pub bytes: Vec<u8>,
}
pub(crate) fn vector(name: &str, id: u8) -> Vector {
    let declaration = super::vectors::vector(name, id);
    let source = super::vectors::source(name);
    let bytes = (0..declaration.upload.bytes)
        .map(|i| match source["pattern"].as_str().expect("pattern") {
            "abc" => b"abc"[usize::try_from(i).expect("index")],
            "linear_mod251" => u8::try_from((i * 31 + 7) % 251).expect("pattern byte"),
            "zeroes" => 0,
            _ => panic!("unsupported fixture pattern"),
        })
        .collect();
    Vector {
        upload: declaration.upload,
        manifest: declaration.manifest,
        bytes,
    }
}
