//! Small application-owned transaction model for local integration evidence only.
use blob_test_protocol::consumer::{AssetView, Failure, Registration, RegistrationSource};
use candid::{CandidType, Deserialize, Principal};
use ic_blob_storage::dto::reference::{
    ReferenceAction, ReferenceCommand, ReferenceReceiptResponse, ReferenceUpload,
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
    admission_started: bool,
    admission_result:
        Option<Result<(), ic_blob_storage::dto::upload::admission::UploadAdmissionFailure>>,
    retain_started: bool,
    retain_result: Option<
        Result<
            ic_blob_storage::dto::reference::ReferenceChange,
            ic_blob_storage::dto::reference::ReferenceTransitionFailure,
        >,
    >,
    upload_state: Option<ic_blob_storage::dto::upload::UploadState>,
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
        let (u, reference) = binding(intent.source);
        let retain_operation = match intent.source {
            RegistrationSource::Existing(r) => Some(r.operation),
            RegistrationSource::Fresh(_) => None,
        };
        if u.service != self.service || u.tenant != self.tenant {
            return Err(Failure::Invalid);
        }
        if let RegistrationSource::Fresh(p) = intent.source
            && [Principal::anonymous(), Principal::management_canister()].contains(&p.uploader)
        {
            return Err(Failure::Invalid);
        }
        if let RegistrationSource::Existing(r) = intent.source
            && (r.action != ReferenceAction::Retain
                || r.reference == u.first_reference
                || r.operation == 0)
        {
            return Err(Failure::Invalid);
        }
        if intent.asset == 0
            || intent.payload.is_empty()
            || intent.payload.len() > 256
            || u.bytes == 0
            || retain_operation == Some(intent.release_operation)
            || [
                u.namespace,
                u.upload,
                u.object,
                u.incarnation,
                u.first_reference,
                reference,
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
            match a.intent.source {
                RegistrationSource::Fresh(_) if a.retain_started || a.retain_result.is_some() => {
                    return Err(Failure::State);
                }
                RegistrationSource::Existing(_)
                    if a.upload_state.is_some()
                        || a.admission_started
                        || a.admission_result.is_some() =>
                {
                    return Err(Failure::State);
                }
                _ => {}
            }
            if a.retain_result.is_some() && !a.retain_started
                || a.admission_result.is_some() && !a.admission_started
                || a.upload_state.is_some() && a.admission_result != Some(Ok(()))
                || a.release_started && (!a.cancelled || !a.owned())
                || a.release_result.is_some() && !a.release_started
                || a.published_once && !a.owned()
                || a.published && (a.cancelled || !a.published_once || !a.owned())
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
            let (old, old_reference) = binding(a.intent.source);
            let (new, new_reference) = binding(intent.source);
            // Reserve this source's operation IDs and reference for the whole object lifetime.
            if old.service == new.service
                && old.tenant == new.tenant
                && old.namespace == new.namespace
                && old.object == new.object
                && old.incarnation == new.incarnation
                && (old_reference == new_reference
                    || operations(&a.intent)
                        .iter()
                        .flatten()
                        .any(|id| operations(intent).contains(&Some(*id))))
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
            admission_started: false,
            admission_result: None,
            retain_started: false,
            retain_result: None,
            upload_state: None,
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
            admission_started: a.admission_started,
            admission_result: a.admission_result,
            retain_started: a.retain_started,
            retain_result: a.retain_result,
            upload_state: a.upload_state,
            release_started: a.release_started,
            release_result: a.release_result,
            uses: a.uses,
        })
    }
    pub(crate) fn command(&self, id: u128, release: bool) -> Result<ReferenceCommand, Failure> {
        let a = self.view(id)?;
        if release {
            let (upload, reference) = binding(a.registration.source);
            Ok(ReferenceCommand {
                upload,
                reference,
                action: ReferenceAction::Release,
                operation: a.registration.release_operation,
            })
        } else {
            match a.registration.source {
                RegistrationSource::Existing(command) => Ok(command),
                RegistrationSource::Fresh(_) => Err(Failure::State),
            }
        }
    }
    pub(crate) fn start(&mut self, id: u128, release: bool) -> Result<bool, Failure> {
        let a = self.asset(id)?;
        if release {
            if !a.cancelled || !a.owned() {
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
            if a.cancelled || a.owned() || a.retain_result.is_some() {
                return Ok(false);
            }
            if matches!(a.intent.source, RegistrationSource::Fresh(_)) {
                if !a.admission_started || matches!(a.admission_result, Some(Err(_))) {
                    return Err(Failure::State);
                }
                return Ok(true);
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
        if a.cancelled || !a.owned() {
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

pub(crate) fn binding(source: RegistrationSource) -> (ReferenceUpload, u128) {
    match source {
        RegistrationSource::Existing(command) => (command.upload, command.reference),
        RegistrationSource::Fresh(permission) => {
            (permission.upload, permission.upload.first_reference)
        }
    }
}
fn operations(intent: &Registration) -> [Option<u128>; 2] {
    [
        Some(intent.release_operation),
        match intent.source {
            RegistrationSource::Existing(r) => Some(r.operation),
            RegistrationSource::Fresh(_) => None,
        },
    ]
}
impl AssetRecord {
    // Historical acquisition evidence; descriptor delivery still checks current liveness.
    fn owned(&self) -> bool {
        match self.intent.source {
            RegistrationSource::Existing(_) => matches!(self.retain_result, Some(Ok(_))),
            RegistrationSource::Fresh(_) => {
                self.upload_state == Some(ic_blob_storage::dto::upload::UploadState::Confirmed)
            }
        }
    }
}
impl ConsumerRecord {
    pub(crate) fn observe_upload(
        &mut self,
        id: u128,
        response: ic_blob_storage::dto::upload::UploadStatusResponse,
    ) -> Result<(), Failure> {
        use ic_blob_storage::dto::upload::UploadState;
        let a = self.asset(id)?;
        if !matches!(a.intent.source, RegistrationSource::Fresh(p) if p.upload == response.upload)
            || a.admission_result != Some(Ok(()))
        {
            return Err(Failure::Conflict);
        }
        if let Some(prior) = a.upload_state {
            let backwards = match prior {
                UploadState::Confirmed | UploadState::Cancelled => prior != response.state,
                UploadState::ExposurePossible => matches!(
                    response.state,
                    UploadState::Reserved | UploadState::Cancelled
                ),
                UploadState::Reserved => false,
            };
            if backwards {
                return Err(Failure::Conflict);
            }
        }
        a.upload_state = Some(response.state);
        Ok(())
    }
    pub(crate) fn owns(&self, id: u128) -> Result<bool, Failure> {
        self.assets
            .iter()
            .find(|a| a.intent.asset == id)
            .map(AssetRecord::owned)
            .ok_or(Failure::Unknown)
    }
}

impl ConsumerRecord {
    pub(crate) fn start_admission(&mut self, id: u128) -> Result<bool, Failure> {
        let a = self.asset(id)?;
        if !matches!(a.intent.source, RegistrationSource::Fresh(_)) || a.cancelled {
            return Err(Failure::State);
        }
        if a.admission_result.is_some() {
            return Ok(false);
        }
        if a.admission_started {
            return Err(Failure::Pending);
        }
        a.admission_started = true;
        Ok(true)
    }
    pub(crate) fn acknowledge_admission(
        &mut self,
        id: u128,
        response: Result<
            ic_blob_storage::dto::upload::admission::UploadAdmissionResponse,
            ic_blob_storage::dto::upload::admission::UploadAdmissionFailure,
        >,
    ) -> Result<(), Failure> {
        let a = self.asset(id)?;
        if !a.admission_started {
            return Err(Failure::State);
        }
        let result = response.map(|_| ());
        if a.admission_result.is_some_and(|old| old != result) {
            return Err(Failure::Conflict);
        }
        if let Ok(response) = response {
            if a.intent.source != RegistrationSource::Fresh(response.permission) {
                return Err(Failure::Conflict);
            }
            a.admission_result = Some(Ok(()));
            self.observe_upload(
                id,
                ic_blob_storage::dto::upload::UploadStatusResponse {
                    upload: response.permission.upload,
                    state: response.state,
                    revoked: response.revoked,
                },
            )?;
        } else {
            a.admission_result = Some(result);
        }
        Ok(())
    }
}
