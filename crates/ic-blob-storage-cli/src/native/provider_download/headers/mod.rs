//! Bounded native response observations, separate from authoritative root metadata.
use reqwest::header::HeaderMap;
use serde::Serialize;

const MAX_HEADERS: usize = 32;
const MAX_BYTES: usize = 8192;
const NAMES: &[&str] = &[
    "content-type",
    "content-length",
    "content-encoding",
    "content-range",
    "cache-control",
    "expires",
    "etag",
    "last-modified",
    "vary",
    "access-control-allow-origin",
    "access-control-allow-credentials",
    "access-control-expose-headers",
    "content-disposition",
    "x-content-type-options",
    "cross-origin-resource-policy",
];

/// Exact observed bytes for one relevant header occurrence, including duplicates.
#[derive(Serialize)]
pub(super) struct HttpHeaderRecord {
    name: &'static str,
    value_bytes: Vec<u8>,
}

/// Completeness applies only to the listed relevant names, not all HTTP headers.
#[derive(Serialize)]
pub(super) struct HttpResponseHeadersRecord {
    complete: bool,
    headers: Vec<HttpHeaderRecord>,
}

pub(super) fn record(input: &HeaderMap) -> HttpResponseHeadersRecord {
    let mut result = HttpResponseHeadersRecord {
        complete: true,
        headers: Vec::new(),
    };
    let mut bytes = 0usize;
    for &name in NAMES {
        for value in input.get_all(name) {
            let count = name.len() + value.as_bytes().len();
            if result.headers.len() == MAX_HEADERS || count > MAX_BYTES - bytes {
                result.complete = false;
                return result;
            }
            bytes += count;
            result.headers.push(HttpHeaderRecord {
                name,
                value_bytes: value.as_bytes().to_vec(),
            });
        }
    }
    result
}

#[cfg(test)]
mod tests;
