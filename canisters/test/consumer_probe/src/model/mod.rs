//! Small application-owned transaction model for local integration evidence only.
use blob_test_protocol::consumer::{AssetView, Failure, Registration};
use candid::{CandidType, Deserialize, Principal};
use ic_blob_storage::dto::reference::{
    ReferenceAction, ReferenceCommand, ReferenceReceiptResponse,
};

#[derive(Clone, CandidType, Deserialize)]
pub(crate) struct ConsumerRecord {
    pub(crate) operator: Principal,
    pub(crate) service: Principal,
    pub(crate) tenant: Principal,
    pub(crate) fenced: bool,
    assets: Vec<AssetRecord>,
}
#[derive(Clone, CandidType, Deserialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Independent application publication and two remote operations can overlap"
)]
struct AssetRecord {
    intent: Registration,
    cancelled: bool,
    published: bool,
    published_once: bool,
    retain_started: bool,
    retain_result: Option<
        Result<
            ic_blob_storage::dto::reference::ReferenceChange,
            ic_blob_storage::dto::reference::ReferenceTransitionFailure,
        >,
    >,
    release_started: bool,
    release_result: Option<
        Result<
            ic_blob_storage::dto::reference::ReferenceChange,
            ic_blob_storage::dto::reference::ReferenceTransitionFailure,
        >,
    >,
    uses: u32,
}
impl ConsumerRecord {
    pub(crate) fn new(operator: Principal, service: Principal, tenant: Principal) -> Self {
        Self {
            operator,
            service,
            tenant,
            fenced: false,
            assets: Vec::new(),
        }
    }
    pub(crate) fn authorize(&self, actor: Principal, mutation: bool) -> Result<(), Failure> {
        if actor != self.operator {
            return Err(Failure::Denied);
        }
        if mutation && self.fenced {
            return Err(Failure::Fenced);
        }
        Ok(())
    }
    fn check(&self, intent: &Registration) -> Result<(), Failure> {
        let r = intent.retain;
        let u = r.upload;
        if u.service != self.service || u.tenant != self.tenant {
            return Err(Failure::Invalid);
        }
        if intent.asset == 0
            || intent.payload.is_empty()
            || intent.payload.len() > 256
            || u.bytes == 0
            || r.action != ReferenceAction::Retain
            || r.reference == u.first_reference
            || intent.release_operation == r.operation
            || [
                u.namespace,
                u.upload,
                u.object,
                u.incarnation,
                u.first_reference,
                r.reference,
                r.operation,
                intent.release_operation,
            ]
            .contains(&0)
        {
            return Err(Failure::Invalid);
        }
        Ok(())
    }
    pub(crate) fn validate(&self) -> Result<(), Failure> {
        if [self.operator, self.service, self.tenant]
            .into_iter()
            .any(|p| p == Principal::anonymous() || p == Principal::management_canister())
        {
            return Err(Failure::Invalid);
        }
        if self.assets.len() > 2 {
            return Err(Failure::Capacity);
        }
        let mut checked = Self::new(self.operator, self.service, self.tenant);
        for a in &self.assets {
            checked.prepare(&a.intent)?;
            if a.retain_result.is_some() && !a.retain_started
                || a.release_started && (!a.cancelled || !matches!(a.retain_result, Some(Ok(_))))
                || a.release_result.is_some() && !a.release_started
                || a.published_once && !matches!(a.retain_result, Some(Ok(_)))
                || a.published
                    && (a.cancelled || !a.published_once || !matches!(a.retain_result, Some(Ok(_))))
                || a.uses > 0 && !a.published
            {
                return Err(Failure::State);
            }
        }
        if checked.assets.len() != self.assets.len() {
            return Err(Failure::Conflict);
        }
        Ok(())
    }
    pub(crate) fn fence(&mut self) {
        self.fenced = true;
    }
    pub(crate) fn prepare(&mut self, intent: &Registration) -> Result<(), Failure> {
        self.check(intent)?;
        if let Some(a) = self.assets.iter().find(|a| a.intent.asset == intent.asset) {
            return if a.intent == *intent {
                Ok(())
            } else {
                Err(Failure::Conflict)
            };
        }
        for a in &self.assets {
            let old = a.intent.retain;
            // Reserve both operation IDs and the reference for the whole object lifetime.
            if old.upload.service == intent.retain.upload.service
                && old.upload.tenant == intent.retain.upload.tenant
                && old.upload.namespace == intent.retain.upload.namespace
                && old.upload.object == intent.retain.upload.object
                && old.upload.incarnation == intent.retain.upload.incarnation
                && (old.reference == intent.retain.reference
                    || [old.operation, a.intent.release_operation]
                        .iter()
                        .any(|id| [intent.retain.operation, intent.release_operation].contains(id)))
            {
                return Err(Failure::Conflict);
            }
        }
        if self.assets.len() == 2 {
            return Err(Failure::Capacity);
        }
        self.assets.push(AssetRecord {
            intent: intent.clone(),
            cancelled: false,
            published: false,
            published_once: false,
            retain_started: false,
            retain_result: None,
            release_started: false,
            release_result: None,
            uses: 0,
        });
        Ok(())
    }
    fn asset(&mut self, id: u128) -> Result<&mut AssetRecord, Failure> {
        self.assets
            .iter_mut()
            .find(|a| a.intent.asset == id)
            .ok_or(Failure::Unknown)
    }
    pub(crate) fn view(&self, id: u128) -> Result<AssetView, Failure> {
        let a = self
            .assets
            .iter()
            .find(|a| a.intent.asset == id)
            .ok_or(Failure::Unknown)?;
        Ok(AssetView {
            registration: a.intent.clone(),
            cancelled: a.cancelled,
            published: a.published,
            published_once: a.published_once,
            retain_started: a.retain_started,
            retain_result: a.retain_result,
            release_started: a.release_started,
            release_result: a.release_result,
            uses: a.uses,
        })
    }
    pub(crate) fn command(&self, id: u128, release: bool) -> Result<ReferenceCommand, Failure> {
        let a = self.view(id)?;
        Ok(if release {
            ReferenceCommand {
                action: ReferenceAction::Release,
                operation: a.registration.release_operation,
                ..a.registration.retain
            }
        } else {
            a.registration.retain
        })
    }
    pub(crate) fn start(&mut self, id: u128, release: bool) -> Result<bool, Failure> {
        let a = self.asset(id)?;
        if release {
            if !a.cancelled || !matches!(a.retain_result, Some(Ok(_))) {
                return Err(Failure::State);
            }
            if a.release_result.is_some() {
                return Ok(false);
            }
            if a.release_started {
                return Err(Failure::Pending);
            }
            a.release_started = true;
        } else {
            if a.cancelled || a.retain_result.is_some() {
                return Ok(false);
            }
            if a.retain_started {
                return Err(Failure::Pending);
            }
            a.retain_started = true;
        }
        Ok(true)
    }
    pub(crate) fn complete(
        &mut self,
        id: u128,
        release: bool,
        receipt: ReferenceReceiptResponse,
    ) -> Result<(), Failure> {
        if self.command(id, release)? != receipt.request {
            return Err(Failure::Conflict);
        }
        let a = self.asset(id)?;
        let (started, result) = if release {
            (a.release_started, &mut a.release_result)
        } else {
            (a.retain_started, &mut a.retain_result)
        };
        if !started {
            return Err(Failure::State);
        }
        if result.is_some_and(|old| old != receipt.result) {
            return Err(Failure::Conflict);
        }
        *result = Some(receipt.result);
        Ok(())
    }
    pub(crate) fn publish(&mut self, id: u128) -> Result<(), Failure> {
        let a = self.asset(id)?;
        if a.cancelled || !matches!(a.retain_result, Some(Ok(_))) {
            return Err(Failure::State);
        }
        a.published = true;
        a.published_once = true;
        Ok(())
    }
    pub(crate) fn cancel(&mut self, id: u128) -> Result<(), Failure> {
        let a = self.asset(id)?;
        if a.uses > 0 {
            return Err(Failure::State);
        }
        // Tombstone and release outbox are one record; its reserved identity is never erased.
        a.cancelled = true;
        a.published = false;
        Ok(())
    }
    pub(crate) fn use_asset(&mut self, id: u128, attach: bool) -> Result<(), Failure> {
        let a = self.asset(id)?;
        if attach {
            if !a.published || a.cancelled {
                return Err(Failure::State);
            }
            a.uses = a.uses.checked_add(1).ok_or(Failure::Capacity)?;
        } else {
            a.uses = a.uses.checked_sub(1).ok_or(Failure::State)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
