//! Typed command grammar; effect handlers retain authority and lifecycle policy.
use crate::native::publish_inputs::{
    MAX_FILE_BYTES, MAX_FILES, MAX_TIMEOUT_SECONDS, MAX_TOTAL_BYTES,
};
use clap::builder::{PossibleValuesParser, StringValueParser, TypedValueParser};
use clap::{Args, Parser, Subcommand, ValueEnum};
use ic_platform::Principal;
use std::{
    fmt::Display,
    num::{NonZeroU64, NonZeroU128},
    path::PathBuf,
    str::FromStr,
};
use url::Url;
// Canonical spelling is product policy; Clap owns invocation, storage and errors.
fn positive<T>(maximum: T) -> impl TypedValueParser<Value = T>
where
    T: FromStr + Display + Ord + Clone + Send + Sync + 'static,
{
    StringValueParser::new().try_map(move |text| {
        let value = crate::native::parsing::positive::<T>(&text)
            .map_err(|_| "invalid canonical positive integer")?;
        if value > maximum {
            Err("value exceeds command limit")
        } else {
            Ok(value)
        }
    })
}
fn principal(text: &str) -> Result<Principal, &'static str> {
    crate::native::parsing::principal(text).map_err(|_| "invalid canonical principal")
}
fn ordinary_principal(text: &str) -> Result<Principal, &'static str> {
    let value = principal(text)?;
    if value == Principal::anonymous() || value == Principal::management_canister() {
        Err("ordinary principal required")
    } else {
        Ok(value)
    }
}
fn file_index(text: &str) -> Result<usize, &'static str> {
    let index = text.parse::<usize>().map_err(|_| "invalid file index")?;
    if index.to_string() != text || index >= crate::native::publish_inputs::MAX_FILES {
        Err("invalid file index")
    } else {
        Ok(index)
    }
}
const UPLOAD_INPUTS_HELP: &str = "Freeze verified bodies and caller-supplied identities offline. No allocation, installed-state proof or provider effect. Partial output is retained and cannot be overwritten.";
const ADMIT_UPLOAD_HELP: &str = "Tenant-authorized update. Persist exact signed intent before one dispatch; no polling or retry. Preserve partial and uncertain journals.";
const UPLOAD_PERMISSION_HELP: &str = "Inspect the exact saved permission. Historical state does not authorize resubmission or establish stored bytes.";
const REFERENCE_RECEIPT_HELP: &str = "Read the exact reference identity without mutation. Observation is not a retention lease or retry permit.";
const UPLOAD_HISTORY_HELP: &str = "One bounded page. Retain a non-null cursor for explicit continuation, including empty filtered pages; no automatic pagination or provider calls.";
const PUBLISH_MAP_HELP: &str = "Authenticate current scope and exact reference liveness. Sequential observations are not a publication lease or provider availability proof; incomplete batches emit no map.";
const PUBLISH_PREPARE_HELP: &str = "At most two original setup updates and four queries per file. Resume binds --source-run and never resubmits a claimed update. Preserve original journals; no certificate or provider transfer.";
const SYNC_GATEWAYS_HELP: &str = "Operator-authorized control with retained intent. Uncertain results require original-state inspection, never automatic redispatch.";
const OBSERVATION_HELP: &str = "Authenticated observation only. Local accounting and provider acknowledgments do not establish independent account credit or authorize a paid effect.";

#[derive(Parser)]
#[command(
    name = "blob-storage",
    disable_version_flag = true,
    args_conflicts_with_subcommands = true,
    arg_required_else_help = true,
    after_long_help = "Use COMMAND --help for operation constraints. New run directories refuse existing or partial output and CR/LF paths. Preserve original journals after uncertainty; never infer retry authority from a missing reply. See docs/operator-guide.md for recovery procedures."
)]
pub(in crate::native) struct Cli {
    #[arg(long, exclusive = true)]
    pub version: bool,
    #[command(subcommand)]
    pub command: Option<Invocation>,
}
#[derive(Subcommand)]
pub(in crate::native) enum Invocation {
    #[command(
        after_long_help = "Offline provider terms only; no signing, linking, funding or financial-cap proof."
    )]
    AccountLinkInputs(AccountLink),
    #[command(
        after_long_help = "Offline validation and retained configuration packets; no deployment, memory grant or provider qualification."
    )]
    InstallationCheck(Installation),
    #[command(after_long_help = UPLOAD_INPUTS_HELP)]
    UploadInputs(UploadInputs),
    #[command(after_long_help = UPLOAD_INPUTS_HELP)]
    PublishInputs(PublishInputs),
    #[command(
        after_long_help = "Encode exact reference/download requests from the saved permission. No identity allocation, mutation, liveness proof or retry authority."
    )]
    ReferenceInputs(ReferenceInputs),
    #[command(after_long_help = ADMIT_UPLOAD_HELP)]
    AdmitUpload(Tenant<Mutation>),
    #[command(
        after_long_help = "Uploader-authorized manifest preparation. Persist exact signed intent before one dispatch; no certificate, provider upload or automatic retry."
    )]
    PrepareUpload(Tenant<Mutation>),
    #[command(after_long_help = ADMIT_UPLOAD_HELP)]
    RevokeUpload(Tenant<Mutation>),
    #[command(after_long_help = UPLOAD_PERMISSION_HELP)]
    UploadPermission(Tenant<Request>),
    #[command(after_long_help = UPLOAD_PERMISSION_HELP)]
    UploadManifest(Tenant<Request>),
    #[command(
        after_long_help = "Submit the exact retained reference command once. Lost replies require receipt inspection; never allocate a replacement operation to retry."
    )]
    SubmitReference(Tenant<Mutation>),
    #[command(after_long_help = REFERENCE_RECEIPT_HELP)]
    ReferenceReceipt(Tenant<Request>),
    #[command(after_long_help = REFERENCE_RECEIPT_HELP)]
    ReferenceStatus(Tenant<Request>),
    #[command(
        after_long_help = "One replicated descriptor and one bounded provider GET. Only root-verified bytes become body.bin; partial output remains body.part. No automatic retry."
    )]
    Download(Tenant<Download>),
    #[command(
        after_long_help = "Compare a local body with the saved declaration. This does not establish provider retention or issue a certificate."
    )]
    VerifyUpload(Tenant<VerifyUpload>),
    #[command(
        after_long_help = "Inspect current issuance eligibility without issuing a certificate. A successful assessment is not retry authority."
    )]
    CertificateAssessment(Tenant<Permission>),
    #[command(after_long_help = UPLOAD_HISTORY_HELP)]
    UploadHistory(Operator<UploadHistory>),
    #[command(after_long_help = OBSERVATION_HELP)]
    Status(Operator<Payer<Empty>>),
    #[command(after_long_help = UPLOAD_HISTORY_HELP)]
    FundingHistory(Operator<Payer<Cursor>>),
    #[command(after_long_help = OBSERVATION_HELP)]
    InspectAccount(Operator<Payer<AccountKind>>),
    #[command(after_long_help = OBSERVATION_HELP)]
    FundingAssessment(Operator<Payer<Amount>>),
    #[command(after_long_help = OBSERVATION_HELP)]
    FundingOutcome(Operator<Payer<Amount>>),
    #[command(after_long_help = SYNC_GATEWAYS_HELP)]
    SyncGateways(Operator<Payer<Directory>>),
    #[command(after_long_help = SYNC_GATEWAYS_HELP)]
    CancelGatewaySync(Operator<Payer<Cancel>>),
    #[command(after_long_help = SYNC_GATEWAYS_HELP)]
    RevokeGateway(Operator<Payer<Revoke>>),
    #[command(
        after_long_help = "Retain bounded capacity/content observations. Existing roots require recovery, retain or retirement handling; no reservation, mutation or publication authority."
    )]
    PublishCheck(Tenant<Checks<{ MAX_FILES as u64 + 1 }>>),
    #[command(after_long_help = PUBLISH_MAP_HELP, after_help = "Requires BLOB_EXPECTED_HOST_RELEASE: the exact deployed Engine release, independent of Client and Contracts versions.")]
    PublishMap(Tenant<Map<{ 2 * MAX_FILES as u64 + 2 }>>),
    #[command(after_long_help = PUBLISH_MAP_HELP, after_help = "Requires BLOB_EXPECTED_HOST_RELEASE: the exact deployed Engine release, independent of Client and Contracts versions.")]
    PublishFileStatus(Tenant<FileStatus>),
    #[command(after_long_help = PUBLISH_PREPARE_HELP)]
    PublishPrepare(Tenant<IndexedPrepare>),
    #[command(after_long_help = PUBLISH_PREPARE_HELP)]
    PublishPrepareResume(Tenant<ResumePrepare>),
    #[command(
        after_long_help = "Serial per-file preparation stops at the first blocker or uncertainty. Never resume the whole batch; use publish-prepare-resume with the original file journal."
    )]
    PublishPrepareBatch(Tenant<Prepare>),
    #[command(
        after_long_help = "Bounded JSON-lines prepare/transfer/status/map session. Retain browser handoff journals for lost replies. Registration is not provider availability; no automatic retry or publication lease.",
        after_help = "Requires BLOB_EXPECTED_HOST_RELEASE: the exact deployed Engine release, independent of Client and Contracts versions."
    )]
    PublishSession(Tenant<Session>),
}
#[derive(Clone, Copy, ValueEnum)]
pub(in crate::native) enum Network {
    Ic,
    Local,
}
impl Network {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ic => "ic",
            Self::Local => "local",
        }
    }
}
#[derive(Args)]
pub(in crate::native) struct Connection {
    #[arg(long)]
    pub network: Network,
    #[arg(long)]
    pub url: Url,
    #[arg(long)]
    pub identity: PathBuf,
    #[arg(long, value_parser = ordinary_principal)]
    pub service: Principal,
    #[arg(long, value_parser = positive(u128::MAX))]
    pub namespace: u128,
    #[arg(long)]
    pub root_key: Option<PathBuf>,
}
#[derive(Args)]
pub(in crate::native) struct Tenant<T: Args> {
    #[command(flatten)]
    pub connection: Connection,
    #[arg(long, value_parser = ordinary_principal)]
    pub actor: Principal,
    #[command(flatten)]
    pub input: T,
}
#[derive(Args)]
pub(in crate::native) struct Operator<T: Args> {
    #[command(flatten)]
    pub connection: Connection,
    #[arg(long = "operator", value_parser = ordinary_principal)]
    pub authority: Principal,
    #[command(flatten)]
    pub input: T,
}
#[derive(Args)]
pub(in crate::native) struct Payer<T: Args> {
    #[arg(long, value_parser = ordinary_principal)]
    pub cashier: Principal,
    #[arg(long = "payer", value_parser = ordinary_principal)]
    pub payment_account: Principal,
    #[command(flatten)]
    pub input: T,
}
#[derive(Args)]
pub(in crate::native) struct Empty {}
#[derive(Args)]
pub(in crate::native) struct Request {
    #[arg(long)]
    pub request: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Mutation {
    #[arg(long)]
    pub request: PathBuf,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Permission {
    #[arg(long)]
    pub permission: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Directory {
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Cursor {
    #[arg(long)]
    pub cursor: Option<PathBuf>,
}
#[derive(Args)]
pub(in crate::native) struct UploadHistory {
    #[arg(long, value_parser = history_filter())]
    pub filter: ic_blob_storage_contracts::dto::upload::history::UploadHistoryFilter,
    #[arg(long)]
    pub cursor: Option<PathBuf>,
}
#[derive(Args)]
pub(in crate::native) struct AccountKind {
    #[arg(long, value_parser = account_kind())]
    pub kind: ic_blob_storage_contracts::dto::account::AccountInspectionKind,
}
#[derive(Args)]
pub(in crate::native) struct Amount {
    #[arg(long, value_parser = positive(u128::MAX))]
    pub operation: u128,
    #[arg(long, value_parser = positive(u128::MAX))]
    pub offered: u128,
    #[arg(long, value_parser = positive(u128::MAX))]
    pub target_balance: Option<u128>,
}
#[derive(Args)]
pub(in crate::native) struct Cancel {
    #[arg(long)]
    pub run_dir: PathBuf,
    #[arg(long, value_parser = positive(u64::MAX))]
    pub sequence: u64,
}
#[derive(Args)]
pub(in crate::native) struct Revoke {
    #[arg(long)]
    pub run_dir: PathBuf,
    #[arg(long, value_parser = ordinary_principal)]
    pub gateway: Principal,
}
#[derive(Args)]
pub(in crate::native) struct Download {
    #[arg(long)]
    pub request: PathBuf,
    #[arg(long)]
    pub project: String,
    #[arg(long)]
    pub gateway: Url,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling")))]
    pub max_bytes: NonZeroU64,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct VerifyUpload {
    #[arg(long)]
    pub permission: PathBuf,
    #[arg(long)]
    pub body: PathBuf,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling")))]
    pub max_bytes: NonZeroU64,
}
#[derive(Args)]
pub(in crate::native) struct Files {
    #[arg(long)]
    pub inputs: PathBuf,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling")))]
    pub max_bytes: NonZeroU64,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_TOTAL_BYTES).expect("positive ceiling")))]
    pub max_total_bytes: NonZeroU64,
    #[arg(long, value_parser = positive(MAX_TIMEOUT_SECONDS))]
    pub timeout_seconds: u64,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Checks<const QUERIES: u64> {
    #[command(flatten)]
    pub files: Files,
    #[arg(long, value_parser = positive(QUERIES))]
    pub max_queries: u64,
}
#[derive(Args)]
pub(in crate::native) struct Map<const QUERIES: u64> {
    #[command(flatten)]
    pub checks: Checks<QUERIES>,
    #[arg(long)]
    pub gateway: Url,
    #[arg(long)]
    pub operator_identity: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct FileStatus {
    #[command(flatten)]
    pub map: Map<4>,
    #[arg(long, value_parser = file_index)]
    pub file_index: usize,
}
#[derive(Args)]
pub(in crate::native) struct Prepare {
    #[command(flatten)]
    pub files: Files,
    #[arg(long)]
    pub uploader_identity: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct IndexedPrepare {
    #[command(flatten)]
    pub prepare: Prepare,
    #[arg(long, value_parser = file_index)]
    pub file_index: usize,
}
#[derive(Args)]
pub(in crate::native) struct ResumePrepare {
    #[command(flatten)]
    pub indexed: IndexedPrepare,
    #[arg(long)]
    pub source_run: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Session {
    #[command(flatten)]
    pub prepare: Prepare,
    #[arg(long)]
    pub gateway: Url,
    #[arg(long)]
    pub operator_identity: PathBuf,
    #[arg(long, value_parser = positive(crate::native::publish_session::budget::step_limit(MAX_FILES).expect("bounded file ceiling")))]
    pub max_steps: u64,
    #[arg(long = "source-session")]
    pub source: Option<PathBuf>,
    #[arg(long)]
    pub browser_selection: Option<PathBuf>,
}
#[derive(Args)]
pub(in crate::native) struct AccountLink {
    #[arg(long, value_parser = principal)]
    pub cashier: Principal,
    #[arg(long, value_parser = principal)]
    pub caller: Principal,
    #[arg(long, value_parser = principal)]
    pub owner: Principal,
    #[arg(long, value_parser = principal)]
    pub payer: Principal,
    #[arg(long, value_parser = positive(NonZeroU128::MAX))]
    pub daily_limit: NonZeroU128,
    #[arg(long, value_parser = positive(NonZeroU64::MAX))]
    pub expiry: NonZeroU64,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Installation {
    #[arg(long)]
    pub configuration: PathBuf,
    #[arg(long, value_parser = principal)]
    pub service: Principal,
    #[arg(long)]
    pub project: String,
    #[arg(long)]
    pub release: String,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct UploadInputs {
    #[arg(long)]
    pub installation: PathBuf,
    #[arg(long)]
    pub binding: PathBuf,
    #[arg(long)]
    pub manifest: PathBuf,
    #[arg(long)]
    pub body: PathBuf,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling")))]
    pub max_bytes: NonZeroU64,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct PublishInputs {
    #[arg(long)]
    pub inventory: PathBuf,
    #[arg(long)]
    pub root: PathBuf,
    #[arg(long)]
    pub installation: PathBuf,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling")))]
    pub max_bytes: NonZeroU64,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_TOTAL_BYTES).expect("positive ceiling")))]
    pub max_total_bytes: NonZeroU64,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct ReferenceInputs {
    #[arg(long)]
    pub permission: PathBuf,
    #[arg(long, value_parser = reference_action())]
    pub action: ic_blob_storage_contracts::dto::reference::ReferenceAction,
    #[arg(long, value_parser = positive(u128::MAX))]
    pub reference: u128,
    #[arg(long, value_parser = positive(u128::MAX))]
    pub operation: u128,
    #[arg(long)]
    pub run_dir: PathBuf,
}
fn account_kind()
-> impl TypedValueParser<Value = ic_blob_storage_contracts::dto::account::AccountInspectionKind> {
    use ic_blob_storage_contracts::dto::account::AccountInspectionKind as Kind;
    PossibleValuesParser::new(["balance", "relationship"]).map(|value| match value.as_str() {
        "balance" => Kind::Balance,
        "relationship" => Kind::PaymentRelationship,
        _ => unreachable!("Clap admits only the declared account kinds"),
    })
}
fn history_filter()
-> impl TypedValueParser<Value = ic_blob_storage_contracts::dto::upload::history::UploadHistoryFilter>
{
    PossibleValuesParser::new(["all", "active", "deletion-pending", "outstanding"]).try_map(
        |value| crate::native::upload_history::filter(&value).map_err(|_| "invalid upload filter"),
    )
}
fn reference_action()
-> impl TypedValueParser<Value = ic_blob_storage_contracts::dto::reference::ReferenceAction> {
    use ic_blob_storage_contracts::dto::reference::ReferenceAction as Action;
    PossibleValuesParser::new(["retain", "release"]).map(|value| match value.as_str() {
        "retain" => Action::Retain,
        "release" => Action::Release,
        _ => unreachable!("Clap admits only the declared reference actions"),
    })
}

#[cfg(test)]
mod tests;
