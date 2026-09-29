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
    pub operator: Principal,
    pub scope: OperatorScope,
    pub root_key: Option<PathBuf>,
}

pub(super) enum Command {
    Status,
    FundingHistory { cursor: Option<PathBuf> },
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
        if !matches!(command, "status" | "funding-history") {
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
        let operator = principal(take("--operator")?)?;
        let service = principal(take("--service")?)?;
        let namespace_text = take("--namespace")?;
        let namespace: u128 = namespace_text.parse().map_err(|_| Failure::Arguments)?;
        if namespace == 0 || namespace.to_string() != namespace_text {
            return Err(Failure::Arguments);
        }
        let cashier = principal(take("--cashier")?)?;
        let payment_account = principal(take("--payer")?)?;
        let root_key = flags.remove("--root-key").map(PathBuf::from);
        let command = match command {
            "funding-history" => Command::FundingHistory {
                cursor: flags.remove("--cursor").map(PathBuf::from),
            },
            _ => Command::Status,
        };
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
            operator,
            scope: OperatorScope {
                service,
                namespace,
                cashier,
                payment_account,
            },
            root_key,
        })
    }
}
