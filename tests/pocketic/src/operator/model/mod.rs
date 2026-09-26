use std::{collections::BTreeMap, net::SocketAddr, str::FromStr};

use candid::Principal;

use super::Failure;

pub(super) struct Command {
    pub check: bool,
    pub target: Target,
}

pub(super) struct Target {
    pub server: SocketAddr,
    pub instance: usize,
    pub canister: Principal,
    pub caller: Principal,
    pub binding: Binding,
}

pub(super) enum Binding {
    Authority(u128),
    Funding(Principal),
}

impl Command {
    pub fn parse(args: &[String]) -> Result<Self, Failure> {
        let check = match args.first().map(String::as_str) {
            Some("status") => false,
            Some("check") => true,
            _ => return Err(Failure::Arguments),
        };
        let mut flags = BTreeMap::new();
        for pair in args[1..].chunks(2) {
            if pair.len() != 2 || flags.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
                return Err(Failure::Arguments);
            }
        }
        let mut take = |key| flags.remove(key).ok_or(Failure::Arguments);
        let server: SocketAddr = take("--server")?.parse().map_err(|_| Failure::Arguments)?;
        if !server.ip().is_loopback() || server.port() == 0 {
            return Err(Failure::Arguments);
        }
        let instance = decimal(take("--instance")?)?;
        let canister = canister(take("--canister")?)?;
        // An explicit simulated caller is not an authenticated production identity.
        let caller = Principal::from_text(take("--caller")?).map_err(|_| Failure::Arguments)?;
        let binding = match take("--kind")? {
            "authority" => Binding::Authority(decimal(take("--namespace")?)?),
            "funding" => Binding::Funding(self::canister(take("--peer")?)?),
            _ => return Err(Failure::Arguments),
        };
        if !flags.is_empty() {
            return Err(Failure::Arguments);
        }
        Ok(Self {
            check,
            target: Target {
                server,
                instance,
                canister,
                caller,
                binding,
            },
        })
    }
}

fn decimal<T: FromStr>(value: &str) -> Result<T, Failure> {
    if value.is_empty()
        || !value.bytes().all(|b| b.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(Failure::Arguments);
    }
    value.parse().map_err(|_| Failure::Arguments)
}

fn canister(value: &str) -> Result<Principal, Failure> {
    let principal = Principal::from_text(value).map_err(|_| Failure::Arguments)?;
    if principal == Principal::anonymous() || principal == Principal::management_canister() {
        return Err(Failure::Arguments);
    }
    Ok(principal)
}

#[cfg(test)]
mod tests;
