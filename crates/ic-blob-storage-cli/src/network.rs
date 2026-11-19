//! Blob's explicit trust, deadlines and observation policy over Platform transport.

use ic_platform::{
    certificate::{PreparedRootKey, TimePolicy, TimeWindow, VerificationLimits},
    native::{
        Dispatch, Ed25519Identity, PreparedUpdate, ReadClient, ReadConfig, ReadError,
        RequestObservation, RequestStatus, UpdateArgs,
    },
};
use ic_platform_types::Principal;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub(crate) const MESSAGE_BYTES: usize = 2 * 1024 * 1024;
pub(crate) struct Client {
    pub(crate) transport: ReadClient,
    identity: Option<Ed25519Identity>,
    pub(crate) root: Vec<u8>,
}

pub(crate) fn now() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "invalid local clock")?
        .as_nanos()
        .try_into()
        .map_err(|_| "local clock exceeds timestamp range".into())
}

pub(crate) fn policy(historical: bool) -> Result<TimePolicy, String> {
    let reference_ns = now()?;
    let certificate = TimeWindow {
        reference_ns,
        max_age_ns: if historical {
            reference_ns
        } else {
            300_000_000_000
        },
        max_future_skew_ns: 30_000_000_000,
    };

    Ok(TimePolicy {
        certificate,
        delegation: TimeWindow {
            max_age_ns: if historical {
                reference_ns
            } else {
                86_400_000_000_000
            },
            ..certificate
        },
    })
}

impl Client {
    pub(crate) fn new(
        url: &str,
        root: Vec<u8>,
        root_subnet: Principal,
        pem: Option<&[u8]>,
    ) -> Result<Self, String> {
        let identity = pem
            .map(|bytes| {
                let text =
                    std::str::from_utf8(bytes).map_err(|_| "invalid identity PEM".to_owned())?;
                Ed25519Identity::from_pem(text, 16 * 1024)
                    .map_err(|_| "invalid Ed25519 identity PEM".to_owned())
            })
            .transpose()?;
        let transport = ReadClient::new(ReadConfig {
            endpoint: url.into(),
            root: PreparedRootKey::from_der(&root).map_err(|_| "invalid trusted root key")?,
            root_subnet,
            message_bytes: MESSAGE_BYTES,
            request_timeout: Duration::from_secs(30),
            verification: VerificationLimits {
                tree_nodes: 4096,
                tree_depth: 64,
                delegation_bytes: 128 * 1024,
                canister_ranges: 256,
            },
        })
        .map_err(|_| "cannot configure IC transport")?;

        Ok(Self {
            transport,
            identity,
            root,
        })
    }

    pub(crate) fn principal(&self) -> Principal {
        self.identity
            .as_ref()
            .map_or(Principal::anonymous(), Ed25519Identity::principal)
    }

    pub(crate) fn signer(&self) -> Result<&Ed25519Identity, String> {
        self.identity
            .as_ref()
            .ok_or_else(|| "this operation requires an explicit identity".into())
    }

    pub(crate) fn prepare(
        &self,
        recipient: Principal,
        effective: Principal,
        method: &str,
        arg: &[u8],
    ) -> Result<PreparedUpdate, String> {
        self.signer()?
            .prepare_update(
                UpdateArgs {
                    recipient,
                    effective,
                    method,
                    arg,
                    expiry: now()?
                        .checked_add(120_000_000_000)
                        .ok_or("expiry overflow")?,
                    nonce: None,
                },
                MESSAGE_BYTES,
            )
            .map_err(|_| "cannot sign original request".into())
    }

    pub(crate) async fn observe(
        &self,
        original: &PreparedUpdate,
    ) -> Result<RequestObservation, String> {
        self.transport
            .observe_request(
                self.signer()?,
                original,
                now()?.saturating_add(120_000_000_000),
                policy(false)?,
            )
            .await
            .map_err(|_| "original request status authentication failed".into())
    }

    pub(crate) async fn update(
        &self,
        recipient: Principal,
        effective: Principal,
        method: &str,
        args: &[u8],
    ) -> Result<Vec<u8>, String> {
        let original = self.prepare(recipient, effective, method, args)?;

        tokio::time::timeout(Duration::from_secs(30), async {
            let mut observed = match self
                .transport
                .dispatch(&original, policy(false)?)
                .await
                .map_err(|error| match error {
                    ReadError::Verification(_) => {
                        "dispatch result authentication failed; no retry attempted"
                    }
                    _ => "dispatch outcome uncertain; no retry attempted",
                })? {
                Dispatch::Observed(value) => Some(value),
                Dispatch::Accepted => None,
                Dispatch::UncertifiedRejection { .. } => {
                    return Err("uncertified dispatch rejection".into());
                }
            };
            loop {
                let value = match observed.take() {
                    Some(value) => value,
                    None => self.observe(&original).await?,
                };
                match value.status() {
                    RequestStatus::Replied(bytes) => return Ok(bytes.clone()),
                    RequestStatus::Rejected { .. } => {
                        return Err("original request rejected".into());
                    }
                    RequestStatus::Done | RequestStatus::Unavailable => {
                        return Err("original outcome unavailable".into());
                    }
                    RequestStatus::Unknown
                    | RequestStatus::Received
                    | RequestStatus::Processing => {
                        tokio::time::sleep(Duration::from_millis(100)).await
                    }
                }
            }
        })
        .await
        .map_err(|_| "original request timed out; no resubmission".to_owned())?
    }

    pub(crate) async fn query(
        &self,
        target: Principal,
        method: &str,
        args: &[u8],
    ) -> Result<Vec<u8>, String> {
        let expiry = now()?.saturating_add(120_000_000_000);
        let time = policy(false)?;

        tokio::time::timeout(Duration::from_secs(30), async {
            match &self.identity {
                Some(signer) => {
                    self.transport
                        .signed_query(signer, target, method, args, expiry, time)
                        .await
                }
                None => {
                    self.transport
                        .query(target, method, args, expiry, time)
                        .await
                }
            }
        })
        .await
        .map_err(|_| "query timed out")?
        .map_err(|error| error.to_string())
    }
}
