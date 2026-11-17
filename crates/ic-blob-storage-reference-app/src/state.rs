//! One bounded application record; no copy of Blob's liveness or accounting state.

use ic_blob_storage::local::persistence::access::Access;
use ic_blob_storage::{
    local::UploadRequest,
    protocol::{Command, RPC_LIMITS},
};
use ic_platform_types::Principal;
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
    access: Access,
}

impl State {
    pub fn create(access: Access, installation: Principal, config: Installation) -> Self {
        validate_config(&config);
        let record = Record {
            version: 1,
            installation,
            config,
            drafts: vec![],
        };
        assert!(
            access
                .read(|view, root| view.is_empty(root).unwrap())
                .unwrap()
        );
        let state = Self { record, access };
        state.persist();
        state
    }

    pub fn open(access: Access, installation: Principal) -> Self {
        let record = Self::load(&access);
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
        Self { record, access }
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

    fn persist(&self) {
        ic_wire::encode(&self.record, ic_wire::Encoding::Payload, LIMITS)
            .expect("catalog aggregate budget");
        let mut rows = vec![(
            vec![0],
            ic_wire::encode(
                &(
                    self.record.version,
                    self.record.installation,
                    &self.record.config,
                ),
                ic_wire::Encoding::Payload,
                INPUT_LIMITS,
            )
            .unwrap(),
        )];
        for draft in &self.record.drafts {
            let mut key = vec![1];
            key.extend_from_slice(&draft.input.id.get().to_be_bytes());
            rows.push((
                key,
                ic_wire::encode(
                    draft,
                    ic_wire::Encoding::Payload,
                    ic_wire::Limits {
                        bytes: 4096,
                        ..INPUT_LIMITS
                    },
                )
                .unwrap(),
            ));
        }
        self.access
            .update(|mut view, root| {
                for (key, bytes) in rows {
                    if view
                        .read()
                        .get(root, &key)
                        .unwrap()
                        .is_none_or(|value| value.as_slice() != bytes)
                    {
                        view.insert(root, &key, &bytes)
                            .expect("native catalog write");
                    }
                }
            })
            .expect("native catalog scope");
    }
    fn load(access: &Access) -> Record {
        access
            .read(|view, root| {
                assert!(view.len(root).unwrap() <= 17);
                let header = view.get(root, &[0]).unwrap().expect("catalog header");
                let (version, installation, config) =
                    ic_wire::decode(header.as_slice(), ic_wire::Encoding::Payload, INPUT_LIMITS)
                        .expect("catalog header format");
                let mut drafts = vec![];
                for entry in view.prefix(root, &[1]).unwrap().entries() {
                    let draft: DraftView = ic_wire::decode(
                        entry.value().as_slice(),
                        ic_wire::Encoding::Payload,
                        ic_wire::Limits {
                            bytes: 4096,
                            ..INPUT_LIMITS
                        },
                    )
                    .expect("catalog draft format");
                    let mut key = vec![1];
                    key.extend_from_slice(&draft.input.id.get().to_be_bytes());
                    assert_eq!(entry.key().as_slice(), key);
                    drafts.push(draft);
                }
                assert_eq!(view.len(root).unwrap(), 1 + drafts.len());
                Record {
                    version,
                    installation,
                    config,
                    drafts,
                }
            })
            .expect("native catalog read")
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
        || ic_wire::encode(input, ic_wire::Encoding::Payload, INPUT_LIMITS).is_err()
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
