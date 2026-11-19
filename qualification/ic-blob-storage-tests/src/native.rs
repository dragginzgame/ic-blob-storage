//! Explicit synthetic signers, trust and bounded observation for local IC tests.

use ic_platform::{
    certificate::{PreparedRootKey, TimePolicy, TimeWindow, VerificationLimits},
    native::{
        Dispatch, Ed25519Identity, PreparedUpdate, ReadClient, ReadConfig, ReadError,
        RequestObservation, RequestStatus, UpdateArgs,
    },
};
use ic_platform_types::Principal;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Public deterministic fixture identities; never a wallet source.
/// # Panics
/// Panics for an unselected fixture seed or invalid fixed PEM.
#[must_use]
pub fn identity(seed: u8) -> Ed25519Identity {
    let body = match seed {
        1 => "MC4CAQAwBQYDK2VwBCIEIAEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEB",
        2 => "MC4CAQAwBQYDK2VwBCIEIAICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIC",
        3 => "MC4CAQAwBQYDK2VwBCIEIAMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMD",
        7 => "MC4CAQAwBQYDK2VwBCIEIAcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcH",
        9 => "MC4CAQAwBQYDK2VwBCIEIAkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJ",
        _ => panic!("unselected public fixture seed"),
    };
    Ed25519Identity::from_pem(
        &format!("-----BEGIN PRIVATE KEY-----\n{body}\n-----END PRIVATE KEY-----\n"),
        4096,
    )
    .unwrap()
}

/// Current test timestamp, with explicit overflow refusal.
/// # Panics
/// Panics if the test clock is outside the nanosecond epoch range.
#[must_use]
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .try_into()
        .unwrap()
}

/// Network observation windows; historical witnesses use an explicit different window.
#[must_use]
pub fn policy() -> TimePolicy {
    let window = TimeWindow {
        reference_ns: now(),
        max_age_ns: 300_000_000_000,
        max_future_skew_ns: 30_000_000_000,
    };
    TimePolicy {
        certificate: window,
        delegation: TimeWindow {
            max_age_ns: 86_400_000_000_000,
            ..window
        },
    }
}

/// One explicitly trusted local network client and synthetic signer.
pub struct Client {
    /// Original Platform transport for exact dispatch and proof checks.
    pub transport: ReadClient,
    signer: Ed25519Identity,
}
impl Client {
    /// Select a simulator endpoint, independently retained root and root subnet.
    /// # Panics
    /// Panics if the fixture endpoint, root, subnet or signer is invalid.
    pub fn new(url: &str, root: &[u8], root_subnet: impl AsRef<[u8]>, seed: u8) -> Self {
        Self {
            signer: identity(seed),
            transport: ReadClient::new(ReadConfig {
                endpoint: url.into(),
                root: PreparedRootKey::from_der(root).unwrap(),
                root_subnet: Principal::from_slice(root_subnet.as_ref()),
                message_bytes: 2 * 1024 * 1024,
                request_timeout: Duration::from_secs(10),
                verification: VerificationLimits {
                    tree_nodes: 4096,
                    tree_depth: 64,
                    delegation_bytes: 128 * 1024,
                    canister_ranges: 256,
                },
            })
            .unwrap(),
        }
    }

    /// Prepare exact signed fixture bytes, without dispatch or renewal.
    /// # Panics
    /// Panics if the fixed fixture request cannot be signed.
    pub fn prepare(&self, target: impl AsRef<[u8]>, method: &str, args: &[u8]) -> PreparedUpdate {
        let target = Principal::from_slice(target.as_ref());
        self.signer
            .prepare_update(
                UpdateArgs {
                    recipient: target,
                    effective: target,
                    method,
                    arg: args,
                    expiry: now() + 120_000_000_000,
                    nonce: None,
                },
                2 * 1024 * 1024,
            )
            .unwrap()
    }

    /// Observe the retained request once, including after ingress expiry.
    pub async fn observe(
        &self,
        original: &PreparedUpdate,
    ) -> Result<RequestObservation, ReadError> {
        self.transport
            .observe_request(&self.signer, original, now() + 120_000_000_000, policy())
            .await
    }

    /// Wait for a terminal observation of one original request, never resubmit it.
    /// # Panics
    /// Panics on authentication failure or expiry of the fixture deadline.
    pub async fn wait(&self, original: &PreparedUpdate) -> RequestObservation {
        tokio::time::timeout(Duration::from_secs(30), async {
            loop {
                let observed = self.observe(original).await.unwrap();
                if matches!(
                    observed.status(),
                    RequestStatus::Replied(_)
                        | RequestStatus::Rejected { .. }
                        | RequestStatus::Done
                ) {
                    return observed;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .expect("fixture original-request deadline")
    }

    /// Send one fixture update and retain authenticated rejection as an error.
    pub async fn update(
        &self,
        target: impl AsRef<[u8]>,
        method: &str,
        args: &[u8],
    ) -> Result<Vec<u8>, ReadError> {
        let original = self.prepare(target, method, args);
        let observed = match self.transport.dispatch(&original, policy()).await? {
            Dispatch::Observed(observed)
                if matches!(
                    observed.status(),
                    RequestStatus::Replied(_)
                        | RequestStatus::Rejected { .. }
                        | RequestStatus::Done
                ) =>
            {
                observed
            }
            Dispatch::Observed(_) => self.wait(&original).await,
            Dispatch::Accepted => self.wait(&original).await,
            Dispatch::UncertifiedRejection { .. } => return Err(ReadError::Packet),
        };
        match observed.status() {
            RequestStatus::Replied(bytes) => Ok(bytes.clone()),
            RequestStatus::Rejected {
                code,
                message,
                error_code,
            } => Err(ReadError::Rejected {
                code: *code,
                message: message.clone(),
                error_code: error_code.clone(),
            }),
            _ => Err(ReadError::MissingPath),
        }
    }

    /// Authenticate a signed fixture query under the selected local root.
    /// # Panics
    /// Panics on expiry of the fixture deadline.
    pub async fn query(
        &self,
        target: impl AsRef<[u8]>,
        method: &str,
        args: &[u8],
    ) -> Result<Vec<u8>, ReadError> {
        tokio::time::timeout(
            Duration::from_secs(30),
            self.transport.signed_query(
                &self.signer,
                Principal::from_slice(target.as_ref()),
                method,
                args,
                now() + 120_000_000_000,
                policy(),
            ),
        )
        .await
        .expect("fixture query deadline")
    }
}
