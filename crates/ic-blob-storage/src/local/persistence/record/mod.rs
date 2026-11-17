//! Canonical records only: indexes, reference counts and byte counters are rebuilt.

use std::collections::{BTreeMap, BTreeSet};

use ic_platform_types::Principal;
use serde::{Deserialize, Serialize};

use super::{OpenError, validate};
use crate::local::{LocalModel, Object, ReferenceStatus, Root, Upload, UploadState, UploadView};
use crate::protocol::{Installation, Maintenance};
use crate::references::{ObjectReferences, ReferenceId, ReferenceState, RetentionState};

pub(super) const STATE_VERSION: u32 = 1;

pub(super) const STATE_LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 65_536,
    allocation: 8 * 1024 * 1024,
    nodes: 262_144,
    depth: 32,
};

#[derive(Deserialize, Serialize)]
pub(super) struct Record {
    version: u32,
    installation: Principal,
    pub input: Installation,
    pub maintenance: Maintenance,
    uploads: Vec<(UploadView, u64)>,
    references: Vec<(ReferenceId, Root, ReferenceStatus)>,
}

impl Record {
    pub fn capture(
        model: &LocalModel,
        manager: Principal,
        installation: Principal,
        attempt: u64,
    ) -> Self {
        Self {
            version: STATE_VERSION,
            installation,
            input: Installation {
                application: model.application,
                manager,
                limits: model.limits,
            },
            maintenance: Maintenance {
                attempt,
                active: model.active,
            },
            uploads: model
                .uploads
                .values()
                .map(|upload| (upload.view, upload.admitted_at))
                .collect(),
            references: model
                .references
                .iter()
                .map(|(&id, &root)| (id, root, model.reference_status(id, root)))
                .collect(),
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, OpenError> {
        ic_wire::encode(self, ic_wire::Encoding::Payload, STATE_LIMITS).map_err(OpenError::Codec)
    }

    pub fn restore(self, installation: Principal) -> Result<LocalModel, OpenError> {
        validate(self.input)?;
        if self.version != STATE_VERSION
            || self.installation != installation
            || (self.maintenance.attempt == 0 && !self.maintenance.active)
            || self.uploads.len() > self.input.limits.uploads.get()
            || self.references.len() > self.input.limits.references.get()
        {
            return Err(OpenError::Record);
        }
        let mut model = LocalModel::new(self.input.application, self.input.limits)
            .map_err(|_| OpenError::Record)?;
        model.active = self.maintenance.active;
        let mut first_references = BTreeSet::new();
        for (view, admitted_at) in self.uploads {
            let request = view.request;
            super::super::valid_principal(request.uploader).map_err(|_| OpenError::Record)?;
            if admitted_at >= request.expires_at
                || request.bytes == 0
                || request.bytes > model.limits.manifest.max_content_bytes.get()
                || !first_references.insert(request.first_reference)
                || model
                    .uploads
                    .insert(request.id, Upload { view, admitted_at })
                    .is_some()
            {
                return Err(OpenError::Record);
            }
            if view.state != UploadState::Attached
                && model
                    .objects
                    .insert(
                        request.root,
                        Object {
                            upload: request.id,
                            references: None,
                        },
                    )
                    .is_some()
            {
                return Err(OpenError::Record);
            }
        }
        let mut registered: BTreeMap<Root, BTreeMap<ReferenceId, ReferenceState>> = BTreeMap::new();
        for (id, root, status) in self.references {
            if model.references.insert(id, root).is_some() {
                return Err(OpenError::Record);
            }
            let object = model.objects.get(&root).ok_or(OpenError::Record)?;
            let original = model.uploads[&object.upload].view;
            match (original.state, status) {
                (UploadState::Reserved, ReferenceStatus::Reserved)
                | (UploadState::Cancelled | UploadState::Expired, ReferenceStatus::Retired)
                    if id == original.request.first_reference => {}
                (UploadState::Registered, ReferenceStatus::Live | ReferenceStatus::Released) => {
                    let state = if status == ReferenceStatus::Live {
                        ReferenceState::Live
                    } else {
                        ReferenceState::Released
                    };
                    registered.entry(root).or_default().insert(id, state);
                }
                _ => return Err(OpenError::Record),
            }
        }
        for (root, object) in &mut model.objects {
            let original = model.uploads[&object.upload].view;
            if original.state == UploadState::Registered {
                object.references = Some(
                    ObjectReferences::from_records(
                        registered.remove(root).ok_or(OpenError::Record)?,
                        model.limits.references,
                    )
                    .ok_or(OpenError::Record)?,
                );
                model.unresolved_bytes = model
                    .unresolved_bytes
                    .checked_add(original.request.bytes)
                    .ok_or(OpenError::Record)?;
                if object.references.as_ref().unwrap().retention_state() == RetentionState::Open {
                    model.live_bytes = model
                        .live_bytes
                        .checked_add(original.request.bytes)
                        .ok_or(OpenError::Record)?;
                }
            } else if original.state == UploadState::Reserved {
                model.reserved_bytes = model
                    .reserved_bytes
                    .checked_add(original.request.bytes)
                    .ok_or(OpenError::Record)?;
            }
        }
        for upload in model.uploads.values() {
            let request = upload.view.request;
            if model.references.get(&request.first_reference) != Some(&request.root) {
                return Err(OpenError::Record);
            }
            let owner = model.objects.get(&request.root).ok_or(OpenError::Record)?;
            let original = model.uploads[&owner.upload].view;
            if upload.view.state == UploadState::Attached
                && (original.state != UploadState::Registered
                    || original.request.bytes != request.bytes)
            {
                return Err(OpenError::Record);
            }
        }
        if model
            .reserved_bytes
            .checked_add(model.live_bytes)
            .is_none_or(|sum| sum > model.limits.logical_bytes.get())
            || model
                .reserved_bytes
                .checked_add(model.unresolved_bytes)
                .is_none_or(|sum| sum > model.limits.exposure_bytes.get())
        {
            return Err(OpenError::Record);
        }
        Ok(model)
    }
}

#[cfg(test)]
mod tests;

#[derive(Deserialize, Serialize)]
struct Header {
    version: u32,
    installation: Principal,
    input: Installation,
    maintenance: Maintenance,
}

impl Record {
    pub fn save(&self, access: &super::Access, fresh: bool) -> Result<(), OpenError> {
        // Retain the finite logical budget, without storing an opaque chunked aggregate.
        self.encode()?;
        let header = Header {
            version: self.version,
            installation: self.installation,
            input: self.input,
            maintenance: self.maintenance,
        };
        let mut rows = vec![(
            vec![0],
            ic_wire::encode(&header, ic_wire::Encoding::Payload, STATE_LIMITS)
                .map_err(OpenError::Codec)?,
        )];
        for &(upload, admitted) in &self.uploads {
            let mut key = vec![1];
            key.extend_from_slice(&upload.request.id.get().to_be_bytes());
            rows.push((
                key,
                ic_wire::encode(
                    &(upload, admitted),
                    ic_wire::Encoding::Payload,
                    STATE_LIMITS,
                )
                .map_err(OpenError::Codec)?,
            ));
        }
        for &(id, root, status) in &self.references {
            let mut key = vec![2];
            key.extend_from_slice(&id.get().to_be_bytes());
            rows.push((
                key,
                ic_wire::encode(&(root, status), ic_wire::Encoding::Payload, STATE_LIMITS)
                    .map_err(OpenError::Codec)?,
            ));
        }
        let changed = access
            .read(|view, root| {
                if fresh && !view.is_empty(root).map_err(storage)? {
                    return Err(OpenError::Record);
                }
                let mut changed = Vec::new();
                for (key, bytes) in rows {
                    let same = view
                        .get(root, &key)
                        .map_err(storage)?
                        .is_some_and(|value| value.as_slice() == bytes);
                    if !same {
                        changed.push((key, bytes));
                    }
                }
                Ok(changed)
            })
            .map_err(OpenError::Storage)??;
        access
            .update(|mut view, root| {
                // Every ordinary refusal precedes publication. An unexpected write
                // failure must unwind within Storage's scope, poisoning a host owner
                // and rolling back the enclosing IC execution.
                for (key, bytes) in changed {
                    view.insert(root, &key, &bytes)
                        .expect("native Blob record publication");
                }
            })
            .map_err(OpenError::Storage)
    }

    pub fn load(access: &super::Access) -> Result<Self, OpenError> {
        access
            .read(|view, root| {
                if view.len(root).map_err(storage)? > 1 + 2 * super::MAX_RECORDS {
                    return Err(OpenError::Record);
                }
                let header = view
                    .get(root, &[0])
                    .map_err(storage)?
                    .ok_or(OpenError::Record)?;
                let header: Header =
                    ic_wire::decode(header.as_slice(), ic_wire::Encoding::Payload, STATE_LIMITS)
                        .map_err(OpenError::Codec)?;
                let mut record = Self {
                    version: header.version,
                    installation: header.installation,
                    input: header.input,
                    maintenance: header.maintenance,
                    uploads: vec![],
                    references: vec![],
                };
                use std::ops::Bound::Unbounded;
                for entry in view
                    .range(root, Unbounded, Unbounded)
                    .map_err(storage)?
                    .entries()
                {
                    let key = entry.key().as_slice();
                    if key == [0] {
                        continue;
                    }
                    if key.len() != 17 {
                        return Err(OpenError::Record);
                    }
                    let id =
                        u128::from_be_bytes(key[1..].try_into().map_err(|_| OpenError::Record)?)
                            .try_into()
                            .map_err(|_| OpenError::Record)?;
                    match key[0] {
                        1 => {
                            let row: (UploadView, u64) = ic_wire::decode(
                                entry.value().as_slice(),
                                ic_wire::Encoding::Payload,
                                STATE_LIMITS,
                            )
                            .map_err(OpenError::Codec)?;
                            if row.0.request.id != id {
                                return Err(OpenError::Record);
                            }
                            record.uploads.push(row);
                        }
                        2 => {
                            let (object, status): (Root, ReferenceStatus) = ic_wire::decode(
                                entry.value().as_slice(),
                                ic_wire::Encoding::Payload,
                                STATE_LIMITS,
                            )
                            .map_err(OpenError::Codec)?;
                            record.references.push((id, object, status));
                        }
                        _ => return Err(OpenError::Record),
                    }
                }
                record.encode()?;
                Ok(record)
            })
            .map_err(OpenError::Storage)?
    }
}
fn storage(error: ic_storage::ordered::session::Error) -> OpenError {
    OpenError::Storage(ic_storage::retained::Error::from(error))
}
