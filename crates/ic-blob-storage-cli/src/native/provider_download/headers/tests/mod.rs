use super::*;
use reqwest::header::HeaderValue;

#[test]
fn observed_values_preserve_duplicates_and_non_text_bytes_without_copying_other_headers() {
    let mut input = HeaderMap::new();
    input.append("vary", HeaderValue::from_static("Origin"));
    input.append("vary", HeaderValue::from_static("Accept-Encoding"));
    input.append("etag", HeaderValue::from_bytes(b"\xff").unwrap());
    input.insert("set-cookie", HeaderValue::from_static("unrelated"));
    let result = record(&input);
    assert!(result.complete);
    assert_eq!(result.headers.len(), 3);
    assert_eq!(result.headers[0].name, "etag");
    assert_eq!(result.headers[0].value_bytes, b"\xff");
    assert_eq!(result.headers[1].value_bytes, b"Origin");
    assert_eq!(result.headers[2].value_bytes, b"Accept-Encoding");
}

#[test]
fn count_and_byte_limits_mark_incomplete_without_truncating_or_inventing_values() {
    let mut input = HeaderMap::new();
    for _ in 0..=MAX_HEADERS {
        input.append("vary", HeaderValue::from_static("Origin"));
    }
    let result = record(&input);
    assert!(!result.complete);
    assert_eq!(result.headers.len(), MAX_HEADERS);
    let mut input = HeaderMap::new();
    input.insert("content-type", HeaderValue::from_static("image/png"));
    input.insert(
        "cache-control",
        HeaderValue::from_bytes(&vec![b'x'; MAX_BYTES]).unwrap(),
    );
    let result = record(&input);
    assert!(!result.complete);
    assert_eq!(result.headers.len(), 1);
    assert_eq!(result.headers[0].value_bytes, b"image/png");
}
