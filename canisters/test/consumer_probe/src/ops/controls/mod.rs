//! Private probe control ingress; service calls keep their exact Contracts codecs.

#[expect(clippy::needless_pass_by_value, reason = "CDK owns the ingress buffer")]
pub(crate) fn decode<T: serde::de::DeserializeOwned>(bytes: Vec<u8>) -> T {
    ic_blob_storage_contracts::document::decode(&bytes, 4096).expect("consumer control ingress")
}

pub(crate) fn encode<T: serde::Serialize>(value: T) -> Vec<u8> {
    ic_blob_storage_contracts::document::encode(value).expect("consumer control reply")
}
