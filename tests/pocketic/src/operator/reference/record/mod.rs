//! Bounded exact fixture intent; stored identity is not an allocation authority.
use crate::operator::{
    Failure,
    model::{canister, decimal},
};
use ic_blob_storage::dto::reference::{ReferenceAction, ReferenceCommand, ReferenceUpload};
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
    first_reference: String,
    root: String,
    bytes: u64,
    reference: String,
    operation: String,
    retain: bool,
}

impl ReferenceIntentRecord {
    pub fn request(&self) -> Result<ReferenceCommand, Failure> {
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
        let root: ProviderRootHash = self.root.parse().map_err(|_| Failure::InvalidRequest)?;
        if root.to_string() != self.root {
            return Err(Failure::InvalidRequest);
        }
        Ok(ReferenceCommand {
            upload: ReferenceUpload {
                service: canister(&self.service).map_err(|_| Failure::InvalidRequest)?,
                tenant: canister(&self.tenant).map_err(|_| Failure::InvalidRequest)?,
                namespace: positive(&self.namespace)?,
                upload: positive(&self.upload)?,
                object: positive(&self.object)?,
                incarnation: positive(&self.incarnation)?,
                first_reference: positive(&self.first_reference)?,
                root: *root.as_bytes(),
                bytes: self.bytes,
            },
            reference: positive(&self.reference)?,
            operation: positive(&self.operation)?,
            action: if self.retain {
                ReferenceAction::Retain
            } else {
                ReferenceAction::Release
            },
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
