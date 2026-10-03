//! Canonical syntax shared by commands; role-specific authority stays with callers.
use super::Failure;
use candid::Principal;
use std::{collections::BTreeMap, fmt::Display, str::FromStr};

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

pub(super) fn principal(value: &str) -> Result<Principal, Failure> {
    let result = Principal::from_text(value).map_err(|_| Failure::Arguments)?;
    if result.to_text() != value {
        return Err(Failure::Arguments);
    }
    Ok(result)
}

pub(super) fn positive<T: FromStr + Display>(value: &str) -> Result<T, Failure> {
    let result = value.parse::<T>().map_err(|_| Failure::Arguments)?;
    if value == "0" || result.to_string() != value {
        return Err(Failure::Arguments);
    }
    Ok(result)
}

#[cfg(test)]
mod tests;
