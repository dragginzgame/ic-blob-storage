//! Reproducible codec-only workloads shared by native and installed probe measurements.
use ic_blob_storage_contracts::dto::{
    reference::ReferenceUpload,
    upload::{
        admission::UploadAdmissionRequest,
        manifest::{UploadManifestDeclaration, UploadManifestHeader, UploadManifestRequest},
    },
};
use ic_platform::Principal;
use ic_wire::{Limits, native};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::hint::black_box;

/// One workload's raw clock samples and deterministic admission charges.
#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    /// Stable workload label.
    pub name: String,
    /// Complete frame identity hash.
    pub frame_sha256: String,
    /// Complete frame length.
    pub frame_bytes: usize,
    /// Minimum accepted allocation allowance for encoding, not heap usage.
    pub encode_charge: usize,
    /// Minimum accepted allocation allowance for decoding, not heap usage.
    pub decode_charge: usize,
    /// Calls per sample; divide raw clock deltas by this count.
    pub repetitions: u64,
    /// Seven encode clock deltas, excluding fixture construction and hashing.
    pub encode_samples: Vec<u64>,
    /// Seven decode clock deltas, including canonical verification and dropping values.
    pub decode_samples: Vec<u64>,
}
fn limits(bytes: usize, allocation: usize) -> Limits {
    Limits {
        bytes,
        allocation,
        nodes: 262_144,
        depth: 32,
    }
}
fn minimum(mut admits: impl FnMut(usize) -> bool) -> usize {
    let (mut low, mut high) = (0, 64 * 1024 * 1024);
    assert!(admits(high));
    while low < high {
        let middle = low + (high - low) / 2;
        if admits(middle) {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    low
}
fn measure<T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(
    name: &str,
    id: u64,
    value: &T,
    max_bytes: usize,
    repetitions: u64,
    clock: &mut impl FnMut() -> u64,
) -> Measurement {
    let budget = limits(max_bytes, 64 * 1024 * 1024);
    let frame = native::encode_request(id, value, budget).unwrap();
    assert_eq!(
        &native::decode_request::<T>(id, &frame, budget).unwrap(),
        value
    );
    let encode_charge = minimum(|allocation| {
        native::encode_request(id, value, limits(max_bytes, allocation)).is_ok()
    });
    let decode_charge = minimum(|allocation| {
        native::decode_request::<T>(id, &frame, limits(max_bytes, allocation)).is_ok()
    });
    let mut encode_samples = Vec::new();
    let mut decode_samples = Vec::new();
    for _ in 0..7 {
        let start = clock();
        for _ in 0..repetitions {
            black_box(native::encode_request(id, black_box(value), budget).unwrap());
        }
        encode_samples.push(clock() - start);
        let start = clock();
        for _ in 0..repetitions {
            black_box(native::decode_request::<T>(id, black_box(&frame), budget).unwrap());
        }
        decode_samples.push(clock() - start);
    }
    Measurement {
        name: name.into(),
        frame_sha256: hex::encode(Sha256::digest(&frame)),
        frame_bytes: frame.len(),
        encode_charge,
        decode_charge,
        repetitions,
        encode_samples,
        decode_samples,
    }
}
/// Measure actual manifest DTOs and a separately labelled opaque byte control.
/// The clock returns native nanoseconds or IC instruction counts; units stay separate.
///
/// # Panics
/// Panics if a fixed workload violates the selected codec contract.
pub fn run(mut clock: impl FnMut() -> u64) -> Vec<Measurement> {
    let mut reports = Vec::new();
    for count in [1_usize, 32, 1024] {
        let request = UploadManifestRequest {
            permission: UploadAdmissionRequest {
                upload: ReferenceUpload {
                    service: Principal::from_slice(&[1, 1]),
                    tenant: Principal::from_slice(&[2, 1]),
                    namespace: u128::MAX - 1,
                    upload: u128::MAX - 2,
                    object: u128::MAX - 3,
                    incarnation: u128::MAX - 4,
                    first_reference: u128::MAX - 5,
                    root: [0x95; 32],
                    bytes: count as u64 * 1024 * 1024,
                },
                uploader: Principal::from_slice(&[3, 1]),
                expires_at_ns: u64::MAX - 1,
            },
            declaration: UploadManifestDeclaration {
                chunks: (0..count)
                    .map(|index| Sha256::digest((index as u64).to_le_bytes()).into())
                    .collect(),
                headers: vec![
                    UploadManifestHeader {
                        name: "Content-Length".into(),
                        value: (count as u64 * 1024 * 1024).to_string(),
                    },
                    UploadManifestHeader {
                        name: "Content-Type".into(),
                        value: "application/octet-stream".into(),
                    },
                ],
            },
        };
        reports.push(measure(
            &format!("manifest-{count}"),
            ic_blob_storage_contracts::wire::identity("blob_prepare_upload").unwrap(),
            &request,
            128 * 1024,
            4,
            &mut clock,
        ));
    }
    let blob = serde_bytes::ByteBuf::from(
        (0..1024 * 1024)
            .map(|i| u8::try_from(i % 251).unwrap())
            .collect::<Vec<_>>(),
    );
    reports.push(measure(
        "opaque-1MiB-control",
        0x626c_6f62_6265_6e63,
        &blob,
        2 * 1024 * 1024,
        4,
        &mut clock,
    ));
    reports
}
