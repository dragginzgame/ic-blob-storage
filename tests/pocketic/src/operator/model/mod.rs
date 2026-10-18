use std::{collections::BTreeMap, net::SocketAddr, str::FromStr};

use ic_platform::Principal;

use super::Failure;

pub(super) struct Command {
    pub check: bool,
    pub target: Target,
}

pub(super) struct Target {
    pub transport: QueryTarget,
    pub binding: Binding,
}

/// Explicit local transport selection, independent of the command's semantic binding.
pub(super) struct QueryTarget {
    pub server: SocketAddr,
    pub instance: usize,
    pub canister: Principal,
    pub caller: Principal,
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
        let mut flags = flags(&args[1..])?;
        let target = Target::parse(&mut flags)?;
        if !flags.is_empty() {
            return Err(Failure::Arguments);
        }
        Ok(Self { check, target })
    }
}

/// Parse each argument pair once; command owners consume their own fields.
pub(super) fn flags(args: &[String]) -> Result<BTreeMap<&str, &str>, Failure> {
    let (pairs, remainder) = args.as_chunks::<2>();
    if !remainder.is_empty() {
        return Err(Failure::Arguments);
    }
    let mut flags = BTreeMap::new();
    for [name, value] in pairs {
        if flags.insert(name.as_str(), value.as_str()).is_some() {
            return Err(Failure::Arguments);
        }
    }
    Ok(flags)
}

impl Target {
    /// Consume explicit transport and binding fields, preserving command arguments.
    pub fn parse(flags: &mut BTreeMap<&str, &str>) -> Result<Self, Failure> {
        let transport = QueryTarget::parse(flags)?;
        let mut take = |key| flags.remove(key).ok_or(Failure::Arguments);
        let binding = match take("--kind")? {
            "authority" => Binding::Authority(decimal(take("--namespace")?)?),
            "funding" => Binding::Funding(canister(take("--peer")?)?),
            _ => return Err(Failure::Arguments),
        };
        Ok(Self { transport, binding })
    }
}

impl QueryTarget {
    /// Consume common transport flags; callers retain their caller/tenant policy.
    pub fn parse(flags: &mut BTreeMap<&str, &str>) -> Result<Self, Failure> {
        let mut take = |key| flags.remove(key).ok_or(Failure::Arguments);
        let server: SocketAddr = take("--server")?.parse().map_err(|_| Failure::Arguments)?;
        if !server.ip().is_loopback() || server.port() == 0 {
            return Err(Failure::Arguments);
        }
        let instance = decimal(take("--instance")?)?;
        let canister = canister(take("--canister")?)?;
        // An explicit simulated caller is not an authenticated production identity.
        let caller = Principal::from_text(take("--caller")?).map_err(|_| Failure::Arguments)?;
        Ok(Self {
            server,
            instance,
            canister,
            caller,
        })
    }
}

pub(super) fn decimal<T: FromStr>(value: &str) -> Result<T, Failure> {
    if value.is_empty()
        || !value.bytes().all(|b| b.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(Failure::Arguments);
    }
    value.parse().map_err(|_| Failure::Arguments)
}

pub(super) fn canister(value: &str) -> Result<Principal, Failure> {
    let principal = Principal::from_text(value).map_err(|_| Failure::Arguments)?;
    if principal == Principal::anonymous() || principal == Principal::management_canister() {
        return Err(Failure::Arguments);
    }
    Ok(principal)
}

#[cfg(test)]
mod tests;
