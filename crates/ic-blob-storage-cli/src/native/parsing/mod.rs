//! Canonical syntax shared by commands; role-specific authority stays with callers.
use super::Failure;
use ic_platform::Principal;
use std::{fmt::Display, str::FromStr};

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
