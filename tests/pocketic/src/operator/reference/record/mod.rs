//! Bounded exact fixture intent; stored identity is not an allocation authority.
use crate::operator::{
    Failure,
    model::{canister, decimal},
};
use blob_test_protocol::admission::{Request, input::ReferenceInput};
use ic_blob_storage::model::identity::ProviderRootHash;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

mod journal;
pub(super) use journal::save;

const MAX_BYTES: u64 = 16 * 1024;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Scope {
    PocketicFixture,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReferenceIntentRecord {
    schema: u8,
    scope: Scope,
    asset: String,
    service: String,
    tenant: String,
    namespace: String,
    upload: String,
    object: String,
    incarnation: String,
    root: String,
    bytes: u64,
    reference: String,
    operation: String,
    retain: bool,
}

impl ReferenceIntentRecord {
    pub fn request(&self) -> Result<ReferenceInput, Failure> {
        if self.schema != 1
            || self.asset.is_empty()
            || self.asset.len() > 256
            || self.asset.chars().any(char::is_control)
            || self.bytes == 0
        {
            return Err(Failure::InvalidRequest);
        }
        let positive = |text: &str| {
            let n: u128 = decimal(text).map_err(|_| Failure::InvalidRequest)?;
            if n == 0 {
                return Err(Failure::InvalidRequest);
            }
            Ok(n)
        };
        let upload = positive(&self.upload)?;
        // The disposable probe fixes object identity to the upload ID and lifetime
        // to one. Keep those bindings explicit in the file, never imply a production allocator.
        if positive(&self.object)? != upload || positive(&self.incarnation)? != 1 {
            return Err(Failure::InvalidRequest);
        }
        let root: ProviderRootHash = self.root.parse().map_err(|_| Failure::InvalidRequest)?;
        if root.to_string() != self.root {
            return Err(Failure::InvalidRequest);
        }
        Ok(ReferenceInput {
            object: Request {
                service: canister(&self.service).map_err(|_| Failure::InvalidRequest)?,
                tenant: canister(&self.tenant).map_err(|_| Failure::InvalidRequest)?,
                namespace: positive(&self.namespace)?,
                id: upload,
                root: *root.as_bytes(),
                bytes: self.bytes,
            },
            reference: positive(&self.reference)?,
            operation: positive(&self.operation)?,
            retain: self.retain,
        })
    }
}

pub(super) fn load(path: &Path) -> Result<ReferenceIntentRecord, Failure> {
    let mut encoded = Vec::new();
    File::open(path)
        .map_err(|_| Failure::Storage)?
        .take(MAX_BYTES + 1)
        .read_to_end(&mut encoded)
        .map_err(|_| Failure::Storage)?;
    if encoded.len() as u64 > MAX_BYTES {
        return Err(Failure::InvalidRequest);
    }
    let record: ReferenceIntentRecord =
        serde_json::from_slice(&encoded).map_err(|_| Failure::InvalidRequest)?;
    record.request()?;
    Ok(record)
}

#[cfg(test)]
mod tests;
