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
    PublishSession(super::publish_session::Input),
    PublishMap(super::publish_map::Input),
    PublishPrepareBatch(super::publish_prepare::Input),
    PublishPrepare(super::publish_prepare::Input),
    PublishCheck(super::publish_check::Input),
    UploadSetup(super::upload_setup::Input),
    Download(super::download::Input),
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
    let principal = super::parsing::principal(value)?;
    if principal == Principal::anonymous() || principal == Principal::management_canister() {
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
                | "publish-check"
                | "publish-map"
                | "publish-file-status"
                | "publish-session"
                | "publish-prepare"
                | "publish-prepare-resume"
                | "publish-prepare-batch"
                | "admit-upload"
                | "prepare-upload"
                | "revoke-upload"
                | "upload-permission"
                | "upload-manifest"
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
                | "download"
                | "submit-attestation"
                | "submit-reference"
        ) {
            return Err(Failure::Arguments);
        }
        let mut flags = super::parsing::flags(&args[1..])?;
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
                    | "publish-check"
                    | "publish-map"
                    | "publish-file-status"
                    | "publish-session"
                    | "publish-prepare"
                    | "publish-prepare-resume"
                    | "publish-prepare-batch"
                    | "admit-upload"
                    | "prepare-upload"
                    | "revoke-upload"
                    | "upload-permission"
                    | "upload-manifest"
                    | "upload-attestation"
                    | "observe-upload"
                    | "download"
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
        let namespace = positive::<u128>(take("--namespace")?)?;
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

use super::parsing::positive;

fn maximum(value: &str) -> Result<std::num::NonZeroU64, Failure> {
    let maximum: std::num::NonZeroU64 = positive(value)?;
    if maximum.get() > super::publish_inputs::MAX_FILE_BYTES {
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
    match command {
        "publish-session" => parse_publish_session(service, namespace, network, flags),
        "publish-check" => Ok(Command::PublishCheck(parse_publish_check(
            service,
            namespace,
            flags,
            super::publish_inputs::MAX_FILES as u128 + 1,
        )?)),
        "publish-map" | "publish-file-status" => {
            let selection = if command == "publish-file-status" {
                let text = flags.remove("--file-index").ok_or(Failure::Arguments)?;
                let index: usize = text.parse().map_err(|_| Failure::Arguments)?;
                if index.to_string() != text || index >= super::publish_inputs::MAX_FILES {
                    return Err(Failure::Arguments);
                }
                super::publish_map::Selection::File(index)
            } else {
                super::publish_map::Selection::Batch
            };
            let gateway = Url::parse(flags.remove("--gateway").ok_or(Failure::Arguments)?)
                .map_err(|_| Failure::Arguments)?;
            validate_url(&gateway, network, network == "local")?;
            Ok(Command::PublishMap(super::publish_map::Input {
                selection,
                operator_identity: PathBuf::from(
                    flags
                        .remove("--operator-identity")
                        .ok_or(Failure::Arguments)?,
                ),
                checks: parse_publish_check(
                    service,
                    namespace,
                    flags,
                    if command == "publish-file-status" {
                        4
                    } else {
                        2 * super::publish_inputs::MAX_FILES as u128 + 2
                    },
                )?,
                gateway,
            }))
        }
        "publish-prepare" | "publish-prepare-resume" | "publish-prepare-batch" => {
            parse_publish_prepare(command, service, namespace, flags)
        }
        _ => parse_scoped_command(command, service, namespace, network, flags),
    }
}

fn parse_scoped_command(
    command: &str,
    service: Principal,
    namespace: u128,
    network: &str,
    flags: &mut BTreeMap<&str, &str>,
) -> Result<Command, Failure> {
    let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
    Ok(
        if let Some(kind) = super::upload_setup::Kind::parse(command) {
            Command::UploadSetup(super::upload_setup::Input {
                kind,
                service,
                namespace,
                request: PathBuf::from(take("--request")?),
                directory: if kind.mutation() {
                    Some(PathBuf::from(take("--run-dir")?))
                } else {
                    None
                },
            })
        } else if matches!(command, "reference-receipt" | "reference-status") {
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
        } else if command == "download" {
            let gateway = Url::parse(take("--gateway")?).map_err(|_| Failure::Arguments)?;
            validate_url(&gateway, network, network == "local")?;
            Command::Download(super::download::Input {
                service,
                namespace,
                request: PathBuf::from(take("--request")?),
                project: take("--project")?.to_owned(),
                gateway,
                directory: PathBuf::from(take("--run-dir")?),
                max_bytes: maximum(take("--max-bytes")?)?,
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

fn parse_publish_check(
    service: Principal,
    namespace: u128,
    flags: &mut BTreeMap<&str, &str>,
    query_limit: u128,
) -> Result<super::publish_check::Input, Failure> {
    let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
    let max_bytes = maximum(take("--max-bytes")?)?;
    let max_total_bytes = positive::<u128>(take("--max-total-bytes")?)?;
    let max_queries = positive::<u128>(take("--max-queries")?)?;
    let timeout_seconds = positive::<u128>(take("--timeout-seconds")?)?;
    if max_total_bytes > u128::from(super::publish_inputs::MAX_TOTAL_BYTES)
        || max_queries > query_limit
        || timeout_seconds > u128::from(super::publish_inputs::MAX_TIMEOUT_SECONDS)
    {
        return Err(Failure::Arguments);
    }
    Ok(super::publish_check::Input {
        service,
        namespace,
        inputs: PathBuf::from(take("--inputs")?),
        directory: PathBuf::from(take("--run-dir")?),
        max_bytes,
        max_total_bytes: std::num::NonZeroU64::new(
            max_total_bytes.try_into().map_err(|_| Failure::Arguments)?,
        )
        .ok_or(Failure::Arguments)?,
        max_queries: max_queries.try_into().map_err(|_| Failure::Arguments)?,
        timeout_seconds: timeout_seconds.try_into().map_err(|_| Failure::Arguments)?,
    })
}

fn parse_publish_prepare(
    command: &str,
    service: Principal,
    namespace: u128,
    flags: &mut BTreeMap<&str, &str>,
) -> Result<Command, Failure> {
    let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
    let index_text = if command == "publish-prepare-batch" {
        "0"
    } else {
        take("--file-index")?
    };
    let index: usize = index_text.parse().map_err(|_| Failure::Arguments)?;
    if index.to_string() != index_text || index >= super::publish_inputs::MAX_FILES {
        return Err(Failure::Arguments);
    }
    let source = if command == "publish-prepare-resume" {
        Some(PathBuf::from(take("--source-run")?))
    } else {
        None
    };
    let mut input = parse_prepare_input(service, namespace, index, flags)?;
    input.source = source;
    Ok(if command == "publish-prepare-batch" {
        Command::PublishPrepareBatch(input)
    } else {
        Command::PublishPrepare(input)
    })
}

fn parse_prepare_input(
    service: Principal,
    namespace: u128,
    index: usize,
    flags: &mut BTreeMap<&str, &str>,
) -> Result<super::publish_prepare::Input, Failure> {
    let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
    let max_bytes = maximum(take("--max-bytes")?)?;
    let total = positive::<u128>(take("--max-total-bytes")?)?;
    let timeout = positive::<u128>(take("--timeout-seconds")?)?;
    if total > u128::from(super::publish_inputs::MAX_TOTAL_BYTES)
        || timeout > u128::from(super::publish_inputs::MAX_TIMEOUT_SECONDS)
    {
        return Err(Failure::Arguments);
    }
    Ok(super::publish_prepare::Input {
        service,
        namespace,
        index,
        max_bytes,
        max_total_bytes: std::num::NonZeroU64::new(
            total.try_into().map_err(|_| Failure::Arguments)?,
        )
        .ok_or(Failure::Arguments)?,
        timeout_seconds: timeout.try_into().map_err(|_| Failure::Arguments)?,
        inputs: PathBuf::from(take("--inputs")?),
        directory: PathBuf::from(take("--run-dir")?),
        uploader_identity: PathBuf::from(take("--uploader-identity")?),
        source: None,
    })
}

fn parse_publish_session(
    service: Principal,
    namespace: u128,
    network: &str,
    flags: &mut BTreeMap<&str, &str>,
) -> Result<Command, Failure> {
    let gateway = Url::parse(flags.remove("--gateway").ok_or(Failure::Arguments)?)
        .map_err(|_| Failure::Arguments)?;
    validate_url(&gateway, network, network == "local")?;
    let max_steps = positive::<u64>(flags.remove("--max-steps").ok_or(Failure::Arguments)?)?;
    if max_steps > 8 * super::publish_inputs::MAX_FILES as u64 + 1 {
        return Err(Failure::Arguments);
    }
    Ok(Command::PublishSession(super::publish_session::Input {
        verifier_identity: flags.remove("--verifier-identity").map(PathBuf::from),
        source_session: flags.remove("--source-session").map(PathBuf::from),
        browser_selection: flags.remove("--browser-selection").map(PathBuf::from),
        operator_identity: PathBuf::from(
            flags
                .remove("--operator-identity")
                .ok_or(Failure::Arguments)?,
        ),
        prepare: parse_prepare_input(service, namespace, 0, flags)?,
        gateway,
        max_steps,
    }))
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
                    positive::<u128>(take("--sequence")?)?
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
            let operation = positive::<u128>(take("--operation")?)?;
            let offered = positive::<u128>(take("--offered")?)?;
            let target_balance = flags
                .remove("--target-balance")
                .map(positive::<u128>)
                .transpose()?;
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
