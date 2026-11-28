//! Bounded Postcard records for durable state.

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{DeserializeOwned, Error as _, SeqAccess, Visitor},
};
use std::{fmt, marker::PhantomData};

/// A record exceeded its byte cap or was invalid.
#[derive(Debug)]
pub enum Error {
    /// The encoded record exceeds the caller's byte limit.
    Bytes,
    /// The record uses an unsupported format version.
    Version,
    /// Bytes remain after the complete record.
    Trailing,
    /// Postcard rejected the record.
    Postcard(postcard::Error),
}

/// Encode into a bounded buffer.
///
/// # Errors
/// Returns a byte-limit or Postcard error.
pub fn encode<T: Serialize>(value: &T, max_bytes: usize) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![0; max_bytes];
    let size = postcard::to_slice(value, &mut bytes)
        .map_err(|error| match error {
            postcard::Error::SerializeBufferFull => Error::Bytes,
            error => Error::Postcard(error),
        })?
        .len();
    bytes.truncate(size);
    Ok(bytes)
}

/// Decode one Postcard record after checking its leading version and byte cap.
///
/// # Errors
/// Refuses an unsupported version, oversized or malformed record, or trailing bytes.
pub fn decode<T: DeserializeOwned>(
    bytes: &[u8],
    max_bytes: usize,
    version: u8,
) -> Result<T, Error> {
    if bytes.len() > max_bytes {
        return Err(Error::Bytes);
    }
    if bytes.first() != Some(&version) {
        return Err(Error::Version);
    }
    let (record, remaining) = postcard::take_from_bytes(bytes).map_err(Error::Postcard)?;
    if !remaining.is_empty() {
        return Err(Error::Trailing);
    }
    Ok(record)
}

/// Decode a durable sequence under its domain count bound before reserving storage.
///
/// # Errors
/// Refuses more than `MAX` elements or an invalid element.
pub fn bounded_vec<'de, D, T, const MAX: usize>(decoder: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct Bounded<T, const MAX: usize>(PhantomData<T>);

    impl<'de, T: Deserialize<'de>, const MAX: usize> Visitor<'de> for Bounded<T, MAX> {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "a sequence of at most {MAX} elements")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Vec<T>, A::Error> {
            let len = sequence
                .size_hint()
                .ok_or_else(|| A::Error::custom("missing record count"))?;
            if len > MAX {
                return Err(A::Error::custom("record count exceeds limit"));
            }
            let mut values = Vec::with_capacity(len);
            while let Some(value) = sequence.next_element()? {
                if values.len() == MAX {
                    return Err(A::Error::custom("record count exceeds limit"));
                }
                values.push(value);
            }
            Ok(values)
        }
    }

    decoder.deserialize_seq(Bounded::<T, MAX>(PhantomData))
}
