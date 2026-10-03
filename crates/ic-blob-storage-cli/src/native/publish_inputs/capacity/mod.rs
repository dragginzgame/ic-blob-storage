//! Fresh-installation arithmetic; the live service remains authoritative.
use super::{Failure, PreparedInput};
use candid::Principal;
use serde_json::{Value, json};
use std::{collections::BTreeSet, num::NonZeroU64};

#[derive(Eq, PartialEq)]
struct Scope {
    service: Principal,
    namespace: u128,
    tenant: Principal,
    uploader: Principal,
    project: String,
    bucket: String,
}

pub(super) struct Capacity {
    scope: Option<Scope>,
    uploads: BTreeSet<u128>,
    objects: BTreeSet<u128>,
    references: BTreeSet<u128>,
    chunks: u64,
    bytes: u128,
    maximum: NonZeroU64,
}
impl Capacity {
    pub fn new(maximum: NonZeroU64) -> Self {
        Self {
            scope: None,
            uploads: BTreeSet::new(),
            objects: BTreeSet::new(),
            references: BTreeSet::new(),
            chunks: 0,
            bytes: 0,
            maximum,
        }
    }
    pub fn add(&mut self, input: &PreparedInput) -> Result<(), Failure> {
        let p = input.permission;
        let scope = Scope {
            service: p.upload.service,
            namespace: p.upload.namespace,
            tenant: p.upload.tenant,
            uploader: p.uploader,
            project: input.project().into(),
            bucket: input.bucket().into(),
        };
        if self
            .scope
            .as_ref()
            .is_some_and(|original| original != &scope)
        {
            return Err(Failure::Binding);
        }
        self.scope = Some(scope);
        if !self.uploads.insert(p.upload.upload)
            || !self.objects.insert(p.upload.object)
            || !self.references.insert(p.upload.first_reference)
        {
            return Err(Failure::Binding);
        }
        self.bytes = self
            .bytes
            .checked_add(u128::from(p.upload.bytes))
            .ok_or(Failure::ReplyLimit)?;
        self.chunks = self
            .chunks
            .checked_add(u64::try_from(input.chunks()).map_err(|_| Failure::ReplyLimit)?)
            .ok_or(Failure::ReplyLimit)?;
        let r = input.resources;
        let files = self.objects.len() as u64;
        if files > u64::from(r.max_objects.min(r.max_tenant_objects))
            || self.chunks > u64::from(r.max_chunks.min(r.max_tenant_chunks))
            || self.bytes
                > r.max_physical_bytes
                    .min(r.max_liability_bytes)
                    .min(r.max_tenant_logical_bytes)
            || self.bytes > u128::from(self.maximum.get())
        {
            return Err(Failure::ReplyLimit);
        }
        Ok(())
    }
    pub fn summary(&self) -> Value {
        json!({"objects":self.objects.len(), "references":self.references.len(),
            "retained_chunks":self.chunks, "physical_bytes":self.bytes.to_string(),
            "liability_bytes":self.bytes.to_string(), "tenant_logical_bytes":self.bytes.to_string(),
            "planned_concurrent_uploads":1, "fresh_installation_fit":true,
            "remaining_capacity_observed":false})
    }
}
