mod schema;
use super::Failure;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_platform::Principal;
use std::path::PathBuf;
use url::{Host, Url};

pub(super) struct Options {
    pub network: &'static str,
    pub url: Url,
    pub identity: PathBuf,
    pub actor: Principal,
    pub root_key: Option<PathBuf>,
}

pub(super) enum Command {
    Metrics {
        service: Principal,
        namespace: u128,
    },
    PublishSession(super::publish_session::Input),
    PublishMap(super::publish_map::Input),
    PublishPrepareBatch(super::publish_prepare::Input),
    PublishPrepare(super::publish_prepare::Input),
    PublishCheck(super::publish_check::Input),
    UploadSetup(super::upload_setup::Input),
    Download(super::download::Input),
    GatewayControl(super::gateway_controls::Input),
    FundingAssessment(
        ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationRequest,
    ),
    InspectAccount(ic_blob_storage_contracts::dto::account::AccountInspectionRequest),
    Reference(super::references::Input),
    FundingOutcome(ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest),
    UploadHistory(super::upload_history::Input),
    CertificateAssessment(super::certificate_assessment::Input),
    SubmitReference(super::submit_reference::Input),
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
}

/// Retrieve a value already admitted by Clap; no reparsing or default substitution.
pub(super) fn value<T: Clone + Send + Sync + 'static>(flags: &clap::ArgMatches, name: &str) -> T {
    flags
        .get_one::<T>(name)
        .expect("required typed CLI argument")
        .clone()
}

impl Options {
    #[cfg(test)]
    pub fn parse(args: &[String]) -> Result<(Command, Self), Failure> {
        let matches = parse(args)?;
        let (command, flags) = matches.subcommand().ok_or(Failure::Arguments)?;
        Self::from_matches(command, flags)
    }
    pub fn from_matches(
        command: &str,
        flags: &clap::ArgMatches,
    ) -> Result<(Command, Self), Failure> {
        let network = value(flags, "--network");
        let url = value(flags, "--url");
        let root_key = flags.get_one::<PathBuf>("--root-key").cloned();
        validate_url(&url, network, root_key.is_some())?;
        let service = value(flags, "--service");
        let namespace = value(flags, "--namespace");
        let command = parse_command(command, service, namespace, network, flags)?;
        let actor = value(
            flags,
            if flags.try_contains_id("--operator").unwrap_or(false) {
                "--operator"
            } else {
                "--actor"
            },
        );
        Ok((
            command,
            Self {
                network,
                url,
                root_key,
                actor,
                identity: value(flags, "--identity"),
            },
        ))
    }
}

fn parse_command(
    command: &str,
    service: Principal,
    namespace: u128,
    network: &str,
    flags: &clap::ArgMatches,
) -> Result<Command, Failure> {
    match command {
        "metrics" => Ok(Command::Metrics { service, namespace }),
        "publish-session" => {
            let gateway = value(flags, "--gateway");
            validate_url(&gateway, network, network == "local")?;
            Ok(Command::PublishSession(super::publish_session::Input {
                source_session: flags.get_one::<PathBuf>("--source-session").cloned(),
                browser_selection: flags.get_one::<PathBuf>("--browser-selection").cloned(),
                operator_identity: value(flags, "--operator-identity"),
                prepare: prepare_input(service, namespace, 0, flags),
                gateway,
                max_steps: value(flags, "--max-steps"),
            }))
        }
        "publish-check" => Ok(Command::PublishCheck(check_input(
            service, namespace, flags,
        ))),
        "publish-map" | "publish-file-status" => {
            let selection = if command == "publish-file-status" {
                super::publish_map::Selection::File(value(flags, "--file-index"))
            } else {
                super::publish_map::Selection::Batch
            };
            let gateway = value(flags, "--gateway");
            validate_url(&gateway, network, network == "local")?;
            Ok(Command::PublishMap(super::publish_map::Input {
                selection,
                operator_identity: value(flags, "--operator-identity"),
                checks: check_input(service, namespace, flags),
                gateway,
            }))
        }
        "publish-prepare" | "publish-prepare-resume" | "publish-prepare-batch" => {
            let index = if command == "publish-prepare-batch" {
                0
            } else {
                value(flags, "--file-index")
            };
            let mut input = prepare_input(service, namespace, index, flags);
            if command == "publish-prepare-resume" {
                input.source = Some(value(flags, "--source-run"));
            }
            Ok(if command == "publish-prepare-batch" {
                Command::PublishPrepareBatch(input)
            } else {
                Command::PublishPrepare(input)
            })
        }
        _ => scoped_command(command, service, namespace, network, flags),
    }
}

fn scoped_command(
    command: &str,
    service: Principal,
    namespace: u128,
    network: &str,
    flags: &clap::ArgMatches,
) -> Result<Command, Failure> {
    Ok(
        if let Some(kind) = super::upload_setup::Kind::parse(command) {
            Command::UploadSetup(super::upload_setup::Input {
                kind,
                service,
                namespace,
                request: value(flags, "--request"),
                directory: if kind.mutation() {
                    Some(value(flags, "--run-dir"))
                } else {
                    None
                },
            })
        } else if let Some(kind) = super::references::Kind::parse(command) {
            Command::Reference(super::references::Input {
                service,
                namespace,
                request: value(flags, "--request"),
                kind,
            })
        } else {
            match command {
                "submit-reference" => Command::SubmitReference(super::submit_reference::Input {
                    service,
                    namespace,
                    request: value(flags, "--request"),
                    directory: value(flags, "--run-dir"),
                }),
                "upload-history" => Command::UploadHistory(super::upload_history::Input {
                    service,
                    namespace,
                    filter: value(flags, "--filter"),
                    cursor: flags.get_one::<PathBuf>("--cursor").cloned(),
                }),
                "certificate-assessment" => {
                    Command::CertificateAssessment(super::certificate_assessment::Input {
                        service,
                        namespace,
                        permission: value(flags, "--permission"),
                    })
                }
                "download" => {
                    let gateway = value(flags, "--gateway");
                    validate_url(&gateway, network, network == "local")?;
                    Command::Download(super::download::Input {
                        service,
                        namespace,
                        request: value(flags, "--request"),
                        project: value(flags, "--project"),
                        gateway,
                        directory: value(flags, "--run-dir"),
                        max_bytes: value(flags, "--max-bytes"),
                    })
                }
                "verify-upload" => Command::VerifyUpload {
                    service,
                    namespace,
                    permission: value(flags, "--permission"),
                    body: value(flags, "--body"),
                    max_bytes: value(flags, "--max-bytes"),
                },
                _ => operator_command(command, service, namespace, flags)?,
            }
        },
    )
}
fn check_input(
    service: Principal,
    namespace: u128,
    flags: &clap::ArgMatches,
) -> super::publish_check::Input {
    super::publish_check::Input {
        service,
        namespace,
        inputs: value(flags, "--inputs"),
        directory: value(flags, "--run-dir"),
        max_bytes: value(flags, "--max-bytes"),
        max_total_bytes: value(flags, "--max-total-bytes"),
        max_queries: value(flags, "--max-queries"),
        timeout_seconds: value(flags, "--timeout-seconds"),
    }
}
fn prepare_input(
    service: Principal,
    namespace: u128,
    index: usize,
    flags: &clap::ArgMatches,
) -> super::publish_prepare::Input {
    super::publish_prepare::Input {
        service,
        namespace,
        index,
        max_bytes: value(flags, "--max-bytes"),
        max_total_bytes: value(flags, "--max-total-bytes"),
        timeout_seconds: value(flags, "--timeout-seconds"),
        inputs: value(flags, "--inputs"),
        directory: value(flags, "--run-dir"),
        uploader_identity: value(flags, "--uploader-identity"),
        source: None,
    }
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
fn operator_command(
    command: &str,
    service: Principal,
    namespace: u128,
    flags: &clap::ArgMatches,
) -> Result<Command, Failure> {
    let scope = OperatorScope {
        service,
        namespace,
        cashier: value(flags, "--cashier"),
        payment_account: value(flags, "--payer"),
    };
    Ok(match command {
        "sync-gateways" | "cancel-gateway-sync" | "revoke-gateway" => {
            let action = match command {
                "sync-gateways" => super::gateway_controls::Action::Sync,
                "cancel-gateway-sync" => {
                    super::gateway_controls::Action::Cancel(value(flags, "--sequence"))
                }
                _ => super::gateway_controls::Action::Revoke(value(flags, "--gateway")),
            };
            Command::GatewayControl(super::gateway_controls::Input {
                scope,
                action,
                directory: value(flags, "--run-dir"),
            })
        }
        "inspect-account" => Command::InspectAccount(
            ic_blob_storage_contracts::dto::account::AccountInspectionRequest {
                scope,
                kind: value(flags, "--kind"),
            },
        ),
        "funding-assessment" => Command::FundingAssessment(
            ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationRequest {
                scope,
                operation: value(flags, "--operation"),
                offered: value(flags, "--offered"),
                target_balance: flags.get_one::<u128>("--target-balance").copied(),
            },
        ),
        "funding-outcome" => Command::FundingOutcome(
            ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest {
                scope,
                operation: value(flags, "--operation"),
                offered: value(flags, "--offered"),
                target_balance: flags.get_one::<u128>("--target-balance").copied(),
            },
        ),
        "funding-history" => Command::FundingHistory {
            scope,
            cursor: flags.get_one::<PathBuf>("--cursor").cloned(),
        },
        "status" => Command::Status { scope },
        _ => return Err(Failure::Arguments),
    })
}

/// Parse once before any file or network effect. Tests use the same grammar.
#[cfg(test)]
pub(super) fn parse(args: &[String]) -> Result<clap::ArgMatches, Failure> {
    schema::cli()
        .try_get_matches_from(
            std::iter::once("blob-storage").chain(args.iter().map(String::as_str)),
        )
        .map_err(|_| Failure::Arguments)
}

/// Clap owns help and syntax errors; refusal never echoes sensitive arguments.
pub(super) fn invocation(args: &[std::ffi::OsString]) -> Result<Option<clap::ArgMatches>, Failure> {
    match schema::cli().try_get_matches_from(
        std::iter::once(std::ffi::OsStr::new("blob-storage"))
            .chain(args.iter().map(std::ffi::OsString::as_os_str)),
    ) {
        Ok(matches) => Ok(Some(matches)),
        Err(error) if error.kind() == clap::error::ErrorKind::DisplayHelp => {
            error.print().map_err(|_| Failure::File)?;
            Ok(None)
        }
        Err(_) => Err(Failure::Arguments),
    }
}
