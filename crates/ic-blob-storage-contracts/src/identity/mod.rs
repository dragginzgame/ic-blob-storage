//! Distinct raw-content and provider identities. Neither confers tenant authority.

pub mod batch;
pub mod caffeine;
pub mod verification;

use std::fmt;
use std::str::FromStr;

use sha2::Digest;
use sha2::Sha256;
use thiserror::Error;

const PREFIX: &str = "sha256:";
const DIGEST_BYTES: usize = 32;

/// SHA-256 of the exact raw content bytes, independent of provider metadata.
///
/// Parsing a claimed digest does not verify content; compare it with [`Self::compute`]
/// or use [`verification::ContentVerifier`] for an ordered stream of bytes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContentDigest([u8; DIGEST_BYTES]);

impl ContentDigest {
    /// Compute SHA-256 over the exact supplied bytes, including empty content.
    #[must_use]
    pub fn compute(content: &[u8]) -> Self {
        Self(Sha256::digest(content).into())
    }

    /// Return the raw SHA-256 digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; DIGEST_BYTES] {
        &self.0
    }
}

impl FromStr for ContentDigest {
    type Err = HashParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_hash(value).map(Self)
    }
}

/// Validate the binary representation of an expected raw digest.
/// This neither hashes content nor establishes the digest's trusted provenance.
impl TryFrom<&[u8]> for ContentDigest {
    type Error = HashParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        bytes
            .try_into()
            .map(Self)
            .map_err(|_| HashParseError::InvalidByteLength {
                actual: bytes.len(),
            })
    }
}

impl fmt::Display for ContentDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_hash(&self.0, formatter)
    }
}

/// Caffeine's 32-byte tree root, which may depend on chunks and metadata.
///
/// This validates the identity's representation only. It does not compute a
/// provider tree or prove that an upload exists or is complete. Its canonical
/// display is `sha256:<64-lowercase-hex>`; parsing accepts either hex case.
/// There is intentionally no conversion from a raw-content digest:
///
/// ```compile_fail
/// use ic_blob_storage_contracts::identity::ContentDigest;
/// use ic_blob_storage_contracts::identity::ProviderRootHash;
/// let root: ProviderRootHash = ContentDigest::compute(b"abc").into();
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderRootHash([u8; DIGEST_BYTES]);

impl ProviderRootHash {
    /// Return the provider root bytes for boundary conversion.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; DIGEST_BYTES] {
        &self.0
    }
}

impl FromStr for ProviderRootHash {
    type Err = HashParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_hash(value).map(Self)
    }
}

impl TryFrom<&[u8]> for ProviderRootHash {
    type Error = HashParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        bytes
            .try_into()
            .map(Self)
            .map_err(|_| HashParseError::InvalidByteLength {
                actual: bytes.len(),
            })
    }
}

impl fmt::Display for ProviderRootHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_hash(&self.0, formatter)
    }
}

/// A malformed hash representation; indices and lengths count bytes, not characters.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum HashParseError {
    /// The input text is empty.
    #[error("hash text is empty")]
    Empty,
    /// The exact lowercase `sha256:` prefix is missing.
    #[error("hash must start with sha256:")]
    InvalidPrefix,
    /// A prefixed text hash must contain exactly 64 hex digits.
    #[error("hash text must be 71 bytes, got {actual}")]
    InvalidTextLength {
        /// Length of the entire text, including its prefix.
        actual: usize,
    },
    /// A binary hash must contain exactly 32 bytes.
    #[error("binary hash must be 32 bytes, got {actual}")]
    InvalidByteLength {
        /// Length of the supplied binary hash.
        actual: usize,
    },
    /// The text contains a non-ASCII-hex byte.
    #[error("non-hex byte at index {index}")]
    InvalidHex {
        /// Offset into the entire text, including its prefix.
        index: usize,
    },
}

fn parse_hash(value: &str) -> Result<[u8; DIGEST_BYTES], HashParseError> {
    if value.is_empty() {
        return Err(HashParseError::Empty);
    }
    let hex = value
        .strip_prefix(PREFIX)
        .ok_or(HashParseError::InvalidPrefix)?;
    if hex.len() != DIGEST_BYTES * 2 {
        return Err(HashParseError::InvalidTextLength {
            actual: value.len(),
        });
    }
    let mut bytes = [0; DIGEST_BYTES];
    for (index, pair) in hex.as_bytes().as_chunks::<2>().0.iter().enumerate() {
        let offset = PREFIX.len() + index * 2;
        bytes[index] = hex_digit(pair[0], offset)? * 16 + hex_digit(pair[1], offset + 1)?;
    }
    Ok(bytes)
}

fn hex_digit(byte: u8, index: usize) -> Result<u8, HashParseError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(HashParseError::InvalidHex { index }),
    }
}

fn format_hash(bytes: &[u8; DIGEST_BYTES], formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(PREFIX)?;
    for byte in bytes {
        write!(formatter, "{byte:02x}")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
