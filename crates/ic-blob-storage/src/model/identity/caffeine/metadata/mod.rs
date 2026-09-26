//! Exact client metadata hashing; this is not HTTP header validation.

use std::{collections::BTreeSet, num::NonZeroUsize};

use sha2::{Digest, Sha256};

use super::{CaffeineHashError, CaffeineHashLimit, CaffeineHeader, Hash};

pub(super) fn hash(
    headers: &[CaffeineHeader<'_>],
    max_headers: NonZeroUsize,
    max_header_bytes: NonZeroUsize,
) -> Result<Option<Hash>, CaffeineHashError> {
    if headers.len() > max_headers.get() {
        return Err(CaffeineHashError::Limit(CaffeineHashLimit::HeaderCount));
    }
    let mut remaining = max_header_bytes.get();
    for header in headers {
        for length in [header.name.len(), header.value.len(), 3] {
            remaining = remaining
                .checked_sub(length)
                .ok_or(CaffeineHashError::Limit(CaffeineHashLimit::HeaderBytes))?;
        }
    }
    if headers.is_empty() {
        return Ok(None);
    }
    let mut names = BTreeSet::new();
    let mut lines = Vec::with_capacity(headers.len());
    for header in headers {
        if !names.insert(header.name) {
            return Err(CaffeineHashError::DuplicateHeader);
        }
        let name = header.name.trim_matches(js_whitespace);
        let value = header.value.trim_matches(js_whitespace);
        lines.push(format!("{name}: {value}\n"));
    }
    // JavaScript default sort compares UTF-16 code units, not Unicode scalars.
    lines.sort_unstable_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
    let mut hasher = Sha256::new();
    hasher.update(b"icfs-metadata/");
    for line in lines {
        hasher.update(line.as_bytes());
    }
    Ok(Some(hasher.finalize().into()))
}

fn js_whitespace(value: char) -> bool {
    // ECMAScript WhiteSpace + LineTerminator: unlike Rust trim, includes FEFF
    // and excludes U+0085. Zs code points are listed rather than locale-dependent.
    matches!(value, '\u{0009}'..='\u{000d}' | '\u{0020}' | '\u{00a0}' |
        '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' |
        '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}
