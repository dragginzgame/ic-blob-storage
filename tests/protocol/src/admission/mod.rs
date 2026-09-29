//! Transient shared-owner probe controls, not a production API or provider protocol.

use crate::journey::{JourneyManifest, JourneyUsage};
use candid::{CandidType, Principal};
use serde::Deserialize;
pub mod input;
pub mod release;

/// Fixed local measurement envelopes; none is a production configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Workload {
    /// Small envelope for authority, metadata and leaf-capacity checks.
    Small,
    /// Two tenants with 128 lifetime admissions each, for history-cost checks.
    RetainedHistory,
    /// 704 confirmed objects, retained manifests and four reference generations.
    ReleaseHistory,
    /// One object with 256 live references and exactly enough release receipts.
    ReferenceHistory,
}

/// Explicit test installation; no provider or account authority is supplied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Installation {
    /// Sole observer/enrollment operator.
    pub operator: Principal,
    /// Fixed bounded workload to install.
    pub workload: Workload,
}

/// Exact fixture operation. Object ID equals operation ID; incarnation/reference are one.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Request {
    /// Claimed service, checked against the actual canister.
    pub service: Principal,
    /// Project that owns admission, independent of the browser uploader.
    pub tenant: Principal,
    /// Claimed local namespace; this fixture installs namespace one.
    pub namespace: u128,
    /// Positive lifetime operation/object ID.
    pub id: u128,
    /// Provider identity.
    pub root: [u8; 32],
    /// Exact reserved length.
    pub bytes: u64,
}

/// Tenant-scoped discovery without a pre-existing operation identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ContentLookup {
    /// Explicit fixture service, checked against the running canister.
    pub service: Principal,
    /// Only this authenticated tenant may discover its content.
    pub tenant: Principal,
    /// Configured namespace, not inferred from a digest.
    pub namespace: u128,
    /// Content root to look up.
    pub root: [u8; 32],
}

/// Current local content state, not a provider observation or a retain receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ContentState {
    /// Admitted without exposure.
    Reserved,
    /// Possible exposure remains unresolved.
    ExposurePossible,
    /// Unexposed cancellation retains its identity.
    Cancelled,
    /// Confirmed with live references.
    Live,
    /// Last reference released; deletion remains unresolved.
    DeletionPending,
    /// Physical deletion known; billing remains unresolved.
    ProviderDeleted,
    /// Final settlement known; identity remains retained.
    Settled,
}

/// Original upload identity and current state, visible only to its tenant.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ContentObservation {
    /// Exact request to recover; discovery does not allocate a new operation.
    pub request: Request,
    /// Local lifecycle at query time.
    pub state: ContentState,
}

/// Tenant-only declaration recovered from local state, not certified HTTP data.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ContentDescriptor {
    /// Exact original identity and current lifecycle; preparation is not completion.
    pub content: ContentObservation,
    /// Original validated name/value pairs, bounded by the installed envelope.
    pub headers: Vec<(String, String)>,
}

/// Immutable project instruction, without caller or clock overrides.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Permission {
    /// Exact local operation.
    pub request: Request,
    /// Only this caller may supply a manifest and mark local exposure.
    pub uploader: Principal,
    /// Exclusive issuance deadline, compared with IC time.
    pub expires_at_ns: u64,
}

/// Passive operator compare-and-set observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Enrollment {
    /// Positive activation generation.
    pub generation: u64,
    /// Whether fresh authority is enabled.
    pub active: bool,
}

/// Local manifest consistency; no content verification or provider completion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ManifestState {
    /// No manifest bound yet.
    Unprepared,
    /// Manifest bound to the declared root and length.
    Bound,
}

/// Observable local catalog phase.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Phase {
    /// Capacity reserved, exposure not marked.
    Reserved,
    /// Possible exposure; bytes remain charged.
    ExposurePossible,
    /// Host completion fact; measurement controls can supply an explicit substitute.
    Confirmed,
    /// Cancelled before exposure, identity history retained.
    Cancelled,
}

/// In-process dispatch only; each update has its own bounded wire input.
/// Exposure returns no certificate and causes no provider effect.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    /// Explicit local host facts and reference mutations for capacity measurements.
    /// These controls are provider substitutes, never a production completion API.
    FixtureLifecycle(release::LifecycleCommand),
    /// Operator-only enrollment with exact precondition.
    Enroll {
        /// Project to update.
        tenant: Principal,
        /// Exact prior observation.
        expected: Option<Enrollment>,
        /// Desired admission state.
        active: bool,
    },
    /// Project admits an uploader.
    Admit(Permission),
    /// Approved uploader binds a bounded manifest.
    Prepare(Request, JourneyManifest),
    /// Root-only local exposure transition, without certificate bytes.
    Expose([u8; 32]),
    /// Project revokes future local issuance.
    Revoke(Request),
}

/// Typed mutation outcome; errors are ordinary Candid replies, never traps.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Outcome {
    /// Original or replayed reference result, including recorded lifecycle failure.
    Reference {
        /// True returns the original receipt without executing again.
        replayed: bool,
        /// Inner result is separate from successful request admission.
        result: Result<bool, release::ReferenceFailure>,
    },
    /// New enrollment observation.
    Enrolled(Enrollment),
    /// Fresh admission.
    Admitted,
    /// Exact admission replay with retained phase.
    Existing(Phase),
    /// Manifest or revocation changed state (false means exact no-op).
    Changed(bool),
    /// Local exposure marked; no certificate or payment was sent.
    Exposed,
}

/// Authenticated read-only observation, excluding bytes and hash checkpoints.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Observation {
    /// Original permission, unchanged by retries.
    pub permission: Permission,
    /// Activation under which admission happened.
    pub generation: u64,
    /// Project has withdrawn local issuance permission.
    pub revoked: bool,
    /// Local exposure/completion state.
    pub phase: Phase,
    /// Manifest declaration state; never a verified-content verdict.
    pub manifest: ManifestState,
    /// This project's charged capacity, disclosed only to the project.
    pub usage: Option<JourneyUsage>,
}

/// Maximum retained diagnostic samples; independent of service history limits.
pub const RESOURCE_SAMPLE_CAPACITY: usize = 32;

/// Completed probe workflow, visible only to its configured operator.
/// Counters are diagnostic observations, not authority or a production cost quote.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ExecutionProfile {
    /// Monotonic diagnostic sequence, used to detect missing samples in a batch.
    /// This transient counter grants no operational or restoration authority.
    pub sequence: u64,
    /// Actual caller of the measured update, including denied calls.
    pub caller: Principal,
    /// Counter at entry to the custom decoder, after CDK argument collection.
    pub before_decode: u64,
    /// Counter after parsing the bounded Candid header/type table.
    pub after_header: u64,
    /// Counter after decoding the command value, before trailing arguments.
    pub after_value: u64,
    /// Counter after bounded decoding and input-buffer disposal, before dispatch.
    pub after_decode: u64,
    /// Message instruction count after decoding and before workflow dispatch.
    pub before_work: u64,
    /// Message instruction count after workflow dispatch, before storing this
    /// observation and serializing the reply. Not the complete ingress cost.
    pub after_work: u64,
}

/// Typed categories used to assert boundary behavior without freezing error prose.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Failure {
    /// Malformed fixture identity or enrollment precondition.
    InvalidInput,
    /// Caller is not the configured operator.
    NotOperator,
    /// Caller is not the project.
    NotProject,
    /// Caller is not the admitted uploader.
    NotUploader,
    /// Caller cannot inspect this operation.
    NotObserver,
    /// Request claims another service.
    WrongService,
    /// Request claims another namespace.
    WrongNamespace,
    /// Tenant has not been enrolled.
    NotEnrolled,
    /// Tenant is currently suspended.
    Suspended,
    /// Permission belongs to an older activation.
    StalePermission,
    /// Changed retry or stale enrollment update.
    Conflict,
    /// Unknown operation or root.
    Unknown,
    /// Permission has been revoked.
    Revoked,
    /// Actual IC time reached the deadline.
    Expired,
    /// Host clock precedes admission.
    ClockReversed,
    /// Configured lifetime or byte envelope exceeded.
    Capacity,
    /// Global retained manifest leaf budget is full.
    ManifestCapacity,
    /// This tenant's retained manifest leaf budget is full.
    TenantManifestCapacity,
    /// Local phase forbids this transition.
    Phase,
    /// A bounded manifest must be bound before exposure.
    Unprepared,
    /// Manifest fails its configured bounds or identity checks.
    Manifest,
    /// Header syntax, uniqueness or declared Content-Length rejected.
    Metadata,
    /// Underlying catalog rejected admission or a transition.
    Catalog,
}
