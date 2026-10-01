use super::Failure;
use candid::Principal;
use ic_blob_storage::dto::operator::OperatorScope;
use std::{collections::BTreeMap, path::PathBuf};
use url::{Host, Url};

pub(super) struct Options {
    pub command: Command,
    pub network: &'static str,
    pub url: Url,
    pub identity: PathBuf,
    pub actor: Principal,
    pub root_key: Option<PathBuf>,
}

pub(super) enum Command {
    GatewayControl(super::gateway_controls::Input),
    FundingAssessment(ic_blob_storage::dto::funding::assessment::FundingPreparationRequest),
    InspectAccount(ic_blob_storage::dto::account::AccountInspectionRequest),
    Reference(super::references::Input),
    FundingOutcome(ic_blob_storage::dto::funding::outcome::FundingOutcomeRequest),
    UploadHistory(super::upload_history::Input),
    CertificateAssessment(super::certificate_assessment::Input),
    SubmitAttestation(super::submit_attestation::Input),
    SubmitReference(super::submit_reference::Input),
    ObserveUpload(super::observe_upload::Input),
    Status {
        scope: OperatorScope,
    },
    FundingHistory {
        scope: OperatorScope,
        cursor: Option<PathBuf>,
    },
    VerifyUpload {
        service: Principal,
        namespace: u128,
        permission: PathBuf,
        body: PathBuf,
        max_bytes: std::num::NonZeroU64,
    },
    UploadAttestation {
        service: Principal,
        namespace: u128,
        verifier: Principal,
        statement: PathBuf,
    },
}

fn principal(value: &str) -> Result<Principal, Failure> {
    let principal = Principal::from_text(value).map_err(|_| Failure::Arguments)?;
    if principal == Principal::anonymous()
        || principal == Principal::management_canister()
        || principal.to_text() != value
    {
        return Err(Failure::Arguments);
    }
    Ok(principal)
}

impl Options {
    pub fn parse(args: &[String]) -> Result<Self, Failure> {
        let command = args.first().map(String::as_str).ok_or(Failure::Arguments)?;
        if !matches!(
            command,
            "status"
                | "inspect-account"
                | "sync-gateways"
                | "cancel-gateway-sync"
                | "revoke-gateway"
                | "upload-history"
                | "certificate-assessment"
                | "funding-history"
                | "funding-outcome"
                | "funding-assessment"
                | "reference-receipt"
                | "reference-status"
                | "verify-upload"
                | "upload-attestation"
                | "observe-upload"
                | "submit-attestation"
                | "submit-reference"
        ) {
            return Err(Failure::Arguments);
        }
        let mut flags = BTreeMap::new();
        for pair in args[1..].chunks(2) {
            if pair.len() != 2 || flags.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
                return Err(Failure::Arguments);
            }
        }
        let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
        let network = match take("--network")? {
            "ic" => "ic",
            "local" => "local",
            _ => return Err(Failure::Arguments),
        };
        let url = Url::parse(take("--url")?).map_err(|_| Failure::Arguments)?;
        let identity = PathBuf::from(take("--identity")?);
        let actor = principal(take(
            if matches!(
                command,
                "verify-upload"
                    | "upload-attestation"
                    | "observe-upload"
                    | "submit-attestation"
                    | "submit-reference"
                    | "certificate-assessment"
                    | "reference-receipt"
                    | "reference-status"
            ) {
                "--actor"
            } else {
                "--operator"
            },
        )?)?;
        let service = principal(take("--service")?)?;
        let namespace = positive(take("--namespace")?)?;
        let command = parse_command(command, service, namespace, network, &mut flags)?;
        let root_key = flags.remove("--root-key").map(PathBuf::from);
        if !flags.is_empty() {
            return Err(Failure::Arguments);
        }
        validate_url(&url, network, root_key.is_some())?;
        Ok(Self {
            command,
            network,
            url,
            identity,
            actor,
            root_key,
        })
    }
}

fn positive(value: &str) -> Result<u128, Failure> {
    let number: u128 = value.parse().map_err(|_| Failure::Arguments)?;
    if number == 0 || number.to_string() != value {
        return Err(Failure::Arguments);
    }
    Ok(number)
}

fn maximum(value: &str) -> Result<std::num::NonZeroU64, Failure> {
    let maximum: std::num::NonZeroU64 = value.parse().map_err(|_| Failure::Arguments)?;
    if maximum.to_string() != value || maximum.get() > 1024 * 1024 * 1024 {
        return Err(Failure::Arguments);
    }
    Ok(maximum)
}
fn parse_command(
    command: &str,
    service: Principal,
    namespace: u128,
    network: &str,
    flags: &mut BTreeMap<&str, &str>,
) -> Result<Command, Failure> {
    let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
    Ok(
        if matches!(command, "reference-receipt" | "reference-status") {
            Command::Reference(super::references::Input {
                service,
                namespace,
                request: PathBuf::from(take("--request")?),
                kind: if command == "reference-receipt" {
                    super::references::Kind::Receipt
                } else {
                    super::references::Kind::Status
                },
            })
        } else if command == "submit-reference" {
            Command::SubmitReference(super::submit_reference::Input {
                service,
                namespace,
                request: PathBuf::from(take("--request")?),
                directory: PathBuf::from(take("--run-dir")?),
            })
        } else if command == "upload-history" {
            Command::UploadHistory(super::upload_history::Input {
                service,
                namespace,
                filter: super::upload_history::filter(take("--filter")?)?,
                cursor: flags.remove("--cursor").map(PathBuf::from),
            })
        } else if command == "certificate-assessment" {
            Command::CertificateAssessment(super::certificate_assessment::Input {
                service,
                namespace,
                permission: PathBuf::from(take("--permission")?),
            })
        } else if command == "submit-attestation" {
            Command::SubmitAttestation(super::submit_attestation::Input {
                service,
                namespace,
                directory: PathBuf::from(take("--run-dir")?),
            })
        } else if command == "observe-upload" {
            let gateway = Url::parse(take("--gateway")?).map_err(|_| Failure::Arguments)?;
            validate_url(&gateway, network, network == "local")?;
            Command::ObserveUpload(super::observe_upload::Input {
                service,
                namespace,
                gateway,
                permission: PathBuf::from(take("--permission")?),
                directory: PathBuf::from(take("--run-dir")?),
                max_bytes: maximum(take("--max-bytes")?)?,
            })
        } else if command == "upload-attestation" {
            Command::UploadAttestation {
                service,
                namespace,
                verifier: principal(take("--verifier")?)?,
                statement: PathBuf::from(take("--statement")?),
            }
        } else if command == "verify-upload" {
            let permission = PathBuf::from(take("--permission")?);
            let body = PathBuf::from(take("--body")?);
            let max_bytes = maximum(take("--max-bytes")?)?;
            Command::VerifyUpload {
                service,
                namespace,
                permission,
                body,
                max_bytes,
            }
        } else {
            parse_operator_command(command, service, namespace, flags)?
        },
    )
}

pub(super) fn validate_url(url: &Url, network: &str, explicit_root: bool) -> Result<(), Failure> {
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
        || url.port() == Some(0)
    {
        return Err(Failure::Arguments);
    }
    let loopback = match url.host() {
        Some(Host::Ipv4(ip)) => ip.is_loopback(),
        Some(Host::Ipv6(ip)) => ip.is_loopback(),
        _ => false,
    };
    let valid_network = match network {
        "local" => loopback && matches!(url.scheme(), "http" | "https") && explicit_root,
        _ => url.scheme() == "https" && url.host().is_some() && !explicit_root,
    };
    if !valid_network {
        return Err(Failure::Arguments);
    }
    Ok(())
}

fn parse_operator_command(
    command: &str,
    service: Principal,
    namespace: u128,
    flags: &mut BTreeMap<&str, &str>,
) -> Result<Command, Failure> {
    let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
    Ok({
        let scope = OperatorScope {
            service,
            namespace,
            cashier: principal(take("--cashier")?)?,
            payment_account: principal(take("--payer")?)?,
        };
        if matches!(
            command,
            "sync-gateways" | "cancel-gateway-sync" | "revoke-gateway"
        ) {
            let action = match command {
                "sync-gateways" => super::gateway_controls::Action::Sync,
                "cancel-gateway-sync" => super::gateway_controls::Action::Cancel(
                    positive(take("--sequence")?)?
                        .try_into()
                        .map_err(|_| Failure::Arguments)?,
                ),
                _ => super::gateway_controls::Action::Revoke(principal(take("--gateway")?)?),
            };
            Command::GatewayControl(super::gateway_controls::Input {
                scope,
                action,
                directory: PathBuf::from(take("--run-dir")?),
            })
        } else if command == "inspect-account" {
            Command::InspectAccount(ic_blob_storage::dto::account::AccountInspectionRequest {
                scope,
                kind: super::account::kind(take("--kind")?)?,
            })
        } else if matches!(command, "funding-outcome" | "funding-assessment") {
            let operation = positive(take("--operation")?)?;
            let offered = positive(take("--offered")?)?;
            let target_balance = flags.remove("--target-balance").map(positive).transpose()?;
            if command == "funding-assessment" {
                return Ok(Command::FundingAssessment(
                    ic_blob_storage::dto::funding::assessment::FundingPreparationRequest {
                        scope,
                        operation,
                        offered,
                        target_balance,
                    },
                ));
            }
            Command::FundingOutcome(
                ic_blob_storage::dto::funding::outcome::FundingOutcomeRequest {
                    scope,
                    operation,
                    offered,
                    target_balance,
                },
            )
        } else if command == "funding-history" {
            Command::FundingHistory {
                scope,
                cursor: flags.remove("--cursor").map(PathBuf::from),
            }
        } else {
            Command::Status { scope }
        }
    })
}
