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
        if !matches!(command, "status" | "funding-history" | "verify-upload") {
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
        let actor = principal(take(if command == "verify-upload" {
            "--actor"
        } else {
            "--operator"
        })?)?;
        let service = principal(take("--service")?)?;
        let namespace_text = take("--namespace")?;
        let namespace: u128 = namespace_text.parse().map_err(|_| Failure::Arguments)?;
        if namespace == 0 || namespace.to_string() != namespace_text {
            return Err(Failure::Arguments);
        }
        let command = if command == "verify-upload" {
            let permission = PathBuf::from(take("--permission")?);
            let body = PathBuf::from(take("--body")?);
            let maximum = take("--max-bytes")?;
            let max_bytes: std::num::NonZeroU64 =
                maximum.parse().map_err(|_| Failure::Arguments)?;
            if max_bytes.to_string() != maximum || max_bytes.get() > 1024 * 1024 * 1024 {
                return Err(Failure::Arguments);
            }
            Command::VerifyUpload {
                service,
                namespace,
                permission,
                body,
                max_bytes,
            }
        } else {
            let scope = OperatorScope {
                service,
                namespace,
                cashier: principal(take("--cashier")?)?,
                payment_account: principal(take("--payer")?)?,
            };
            if command == "funding-history" {
                Command::FundingHistory {
                    scope,
                    cursor: flags.remove("--cursor").map(PathBuf::from),
                }
            } else {
                Command::Status { scope }
            }
        };
        let root_key = flags.remove("--root-key").map(PathBuf::from);
        if !flags.is_empty()
            || !url.username().is_empty()
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
            "local" => loopback && matches!(url.scheme(), "http" | "https") && root_key.is_some(),
            _ => url.scheme() == "https" && url.host().is_some() && root_key.is_none(),
        };
        if !valid_network {
            return Err(Failure::Arguments);
        }
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
