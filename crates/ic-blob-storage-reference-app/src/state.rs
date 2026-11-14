//! One bounded application record; no copy of Blob's liveness or accounting state.

use ic_blob_storage::{
    local::UploadRequest,
    protocol::{Command, RPC_LIMITS},
};
use ic_platform_types::Principal;
use ic_storage::{DefaultMemoryImpl, RuntimeMemory, adapter::StableCell};
use serde::{Deserialize, Serialize};

use crate::protocol::{DraftInput, DraftView, Error, Installation, Phase};

pub(crate) const LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 65_536,
    ..RPC_LIMITS
};
const INPUT_LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 2048,
    ..RPC_LIMITS
};

#[derive(Deserialize, Serialize)]
struct Record {
    version: u32,
    installation: Principal,
    config: Installation,
    drafts: Vec<DraftView>,
}

pub(crate) struct State {
    record: Record,
    cell: StableCell<Vec<u8>, DefaultMemoryImpl>,
}

impl State {
    pub fn create(
        memory: RuntimeMemory<DefaultMemoryImpl>,
        installation: Principal,
        config: Installation,
    ) -> Self {
        validate_config(&config);
        let record = Record {
            version: 1,
            installation,
            config,
            drafts: vec![],
        };
        let cell =
            StableCell::create(memory, ic_wire::native::encode(&record, LIMITS).unwrap()).unwrap();
        Self { record, cell }
    }

    pub fn open(memory: RuntimeMemory<DefaultMemoryImpl>, installation: Principal) -> Self {
        let cell = StableCell::<Vec<u8>, _>::load(memory, LIMITS.bytes)
            .expect("existing application record");
        let record: Record =
            ic_wire::native::decode(cell.get(), LIMITS).expect("bounded application record");
        assert_eq!(record.version, 1, "application format");
        assert_eq!(record.installation, installation, "application identity");
        validate_config(&record.config);
        assert!(record.drafts.len() <= 16);
        for (index, draft) in record.drafts.iter().enumerate() {
            assert!(record.config.users.contains(&draft.uploader));
            assert!(validate_input(&draft.input).is_ok());
            for other in &record.drafts[..index] {
                assert_ne!(draft.input.id, other.input.id);
                assert!(
                    !(draft.phase == Phase::Published
                        && other.phase == Phase::Published
                        && draft.uploader == other.uploader
                        && draft.input.asset == other.input.asset)
                );
            }
        }
        Self { record, cell }
    }

    pub fn blob(&self) -> Principal {
        self.record.config.blob
    }

    pub fn catalog(&self, caller: Principal) -> Result<Vec<DraftView>, Error> {
        self.user(caller)?;
        Ok(self
            .record
            .drafts
            .iter()
            .filter(|draft| draft.uploader == caller && draft.phase == Phase::Published)
            .cloned()
            .collect())
    }

    pub fn view(&self, caller: Principal, id: std::num::NonZeroU128) -> Result<DraftView, Error> {
        self.user(caller)?;
        let draft = self
            .record
            .drafts
            .iter()
            .find(|draft| draft.input.id == id)
            .ok_or(Error::Unknown)?;
        if draft.uploader != caller {
            return Err(Error::Denied);
        }
        Ok(draft.clone())
    }

    pub fn save(&mut self, caller: Principal, input: DraftInput) -> Result<DraftView, Error> {
        self.user(caller)?;
        if self
            .record
            .drafts
            .iter()
            .any(|draft| draft.input.id == input.id)
        {
            let draft = self.view(caller, input.id)?;
            return if draft.input == input {
                Ok(draft)
            } else {
                Err(Error::Conflict)
            };
        }
        validate_input(&input)?;
        if self.record.drafts.len() == 16 {
            return Err(Error::Capacity);
        }
        let draft = DraftView {
            input,
            uploader: caller,
            phase: Phase::Draft,
        };
        self.record.drafts.push(draft.clone());
        self.persist();
        Ok(draft)
    }

    pub fn publish(
        &mut self,
        caller: Principal,
        id: std::num::NonZeroU128,
    ) -> Result<DraftView, Error> {
        let draft = self.view(caller, id)?;
        if matches!(draft.phase, Phase::Cancelled | Phase::Cleaned) {
            return Err(Error::Cancelled);
        }
        if self.record.drafts.iter().any(|other| {
            other.input.id != id
                && other.uploader == caller
                && other.input.asset == draft.input.asset
                && other.phase == Phase::Published
        }) {
            return Err(Error::Conflict);
        }
        self.set_phase(id, Phase::Published);
        self.view(caller, id)
    }

    pub fn cancel(
        &mut self,
        caller: Principal,
        id: std::num::NonZeroU128,
    ) -> Result<DraftView, Error> {
        let draft = self.view(caller, id)?;
        if draft.phase != Phase::Cleaned {
            self.set_phase(id, Phase::Cancelled);
        }
        self.view(caller, id)
    }

    pub fn cleaned(
        &mut self,
        caller: Principal,
        id: std::num::NonZeroU128,
    ) -> Result<DraftView, Error> {
        let draft = self.view(caller, id)?;
        assert!(matches!(draft.phase, Phase::Cancelled | Phase::Cleaned));
        self.set_phase(id, Phase::Cleaned);
        self.view(caller, id)
    }

    fn user(&self, caller: Principal) -> Result<(), Error> {
        if self.record.config.users.contains(&caller) {
            Ok(())
        } else {
            Err(Error::Denied)
        }
    }

    fn set_phase(&mut self, id: std::num::NonZeroU128, phase: Phase) {
        self.record
            .drafts
            .iter_mut()
            .find(|draft| draft.input.id == id)
            .unwrap()
            .phase = phase;
        self.persist();
    }

    fn persist(&mut self) {
        // A failure traps this message; never turn it into an ordinary error after mutation.
        self.cell.set(
            ic_wire::native::encode(&self.record, LIMITS).expect("bounded application persistence"),
        );
    }
}

pub(crate) fn request(draft: &DraftView) -> UploadRequest {
    UploadRequest {
        id: draft.input.id,
        first_reference: draft.input.id,
        uploader: draft.uploader,
        root: draft.input.root,
        bytes: draft.input.bytes,
        expires_at: draft.input.expires_at,
    }
}

pub(crate) fn authorization(draft: &DraftView) -> Command {
    Command::Authorize {
        request: request(draft),
        chunks: draft.input.chunks.clone(),
        headers: draft.input.headers.clone(),
    }
}

fn validate_config(config: &Installation) {
    assert!(valid_principal(config.blob));
    assert!(!config.users.is_empty() && config.users.len() <= 8);
    for (index, user) in config.users.iter().enumerate() {
        assert!(valid_principal(*user) && !config.users[..index].contains(user));
    }
}

fn valid_principal(value: Principal) -> bool {
    value != Principal::anonymous() && value != Principal::management_canister()
}

fn validate_input(input: &DraftInput) -> Result<(), Error> {
    if input.asset.is_empty()
        || input.asset.len() > 64
        || input.bytes == 0
        || input.expires_at == 0
        || input.chunks.is_empty()
        || input.chunks.len() > 8
        || input.headers.len() > 8
        || ic_wire::native::encode(input, INPUT_LIMITS).is_err()
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
