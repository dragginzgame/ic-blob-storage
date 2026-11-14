//! Fixed first-party SDK observations; see the provider review's vector provenance.
//! This fixture does not prepare arbitrary uploads or implement SDK transfer code.

use ic_blob_storage::caffeine::{Header, ManifestLimits};
use ic_blob_storage::local::{Limits, Root, UploadRequest};
use ic_platform_types::Principal;

pub const BYTES: usize = 5 * 1024 * 1024 + 17;
pub const HEADERS: [Header<'static>; 2] = [
    Header {
        name: "Content-Length",
        value: "5242897",
    },
    Header {
        name: "Content-Type",
        value: "application/octet-stream",
    },
];

pub fn body() -> Vec<u8> {
    (0..BYTES).map(|i| ((i * 31 + 7) % 251) as u8).collect()
}

pub fn request(number: u128, uploader: Principal) -> UploadRequest {
    UploadRequest {
        id: number.try_into().unwrap(),
        uploader,
        root: hash("bc608ea2567f6809b872afc8e9ec44fc0addf3fd58a00e7f7aa7757b35927a03"),
        bytes: BYTES as u64,
        first_reference: (100 + number).try_into().unwrap(),
        expires_at: 100,
    }
}

pub fn chunks() -> [Root; 6] {
    [
        "f6ecd22dc439c229d43a9abf28813e3ab5775ddea33dd7cc22e59a4a0bfed925",
        "bca62763243aeeae8e10de85e7161e5932e30c99c01beaa1dbdd844158c3ba87",
        "d8d64a80aa7b2ed2b37e9d4c9e268c8f3f00888fae73f8dd559c43b7853cf703",
        "e94b04a2cd33555c8152c334616a0628fceccd89910d90df2730691023d57eec",
        "55a2198e0c31902dc19f1deced6fa8bab331c6445c7dcd1dfdcbe6ff4b0cfeb8",
        "7840422043ff9041ab2fcd0bff33f6d245e077f5a6a5616aa44263d17234aeb6",
    ]
    .map(hash)
}

// Separate frozen "abc" vector: cancelling it permanently retires that root.
pub fn cancelled_request(uploader: Principal) -> UploadRequest {
    UploadRequest {
        root: hash("b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7"),
        bytes: 3,
        ..request(1, uploader)
    }
}

pub fn limits() -> Limits {
    Limits {
        manifest: ManifestLimits {
            max_content_bytes: (BYTES as u64).try_into().unwrap(),
            max_chunks: 6_usize.try_into().unwrap(),
            max_headers: 2_usize.try_into().unwrap(),
            max_header_bytes: 128_usize.try_into().unwrap(),
        },
        uploads: 6_usize.try_into().unwrap(),
        references: 6_usize.try_into().unwrap(),
        logical_bytes: (BYTES as u64).try_into().unwrap(),
        exposure_bytes: (BYTES as u64).try_into().unwrap(),
    }
}

fn hash(value: &str) -> Root {
    std::array::from_fn(|i| {
        u8::from_str_radix(&value[2 * i..2 * i + 2], 16).expect("fixed SDK vector")
    })
}
