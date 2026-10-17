//! Canonical lossless decimal text for provider integers wider than CBOR u128.
use serde::{Deserialize, Deserializer, Serializer, de::Error};
use std::{fmt::Display, str::FromStr};

pub(crate) fn serialize<T: Display, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}
pub(crate) fn deserialize<'de, T: FromStr + Display, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    let text = String::deserialize(deserializer)?;
    let value = text
        .parse::<T>()
        .map_err(|_| D::Error::custom("invalid provider integer"))?;
    if value.to_string() != text {
        return Err(D::Error::custom("noncanonical provider integer"));
    }
    Ok(value)
}
