use super::*;

fn positive(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("positive budget")
}

fn limits() -> AuditLogReplyLimits {
    AuditLogReplyLimits {
        max_bytes: positive(4096),
        max_csv_bytes: positive(1024),
        max_reported_entries: NonZeroU64::new(10).expect("positive entry bound"),
        decoding_quota: positive(100_000),
        skipping_quota: positive(1000),
        max_type_entries: positive(32),
    }
}

fn fixture(hex: &str) -> Vec<u8> {
    let (pairs, remainder) = hex.trim().as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty(), "complete fixture hex bytes");
    pairs
        .iter()
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII hex"), 16)
                .expect("fixture hex byte")
        })
        .collect()
}

const PAGE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/caffeine-audit/page.hex"
));
const EMPTY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/caffeine-audit/empty.hex"
));

fn page(bytes: &[u8]) -> AuditLogPageView {
    match decode_audit_log_reply(bytes, limits()).expect("bounded report") {
        AuditLogReply::ReportedPage(page) => page,
        other @ AuditLogReply::ProviderFailure(_) => panic!("unexpected reply: {other:?}"),
    }
}

#[test]
fn independent_pages_preserve_opaque_csv_and_cursor_fields() {
    let report = page(&fixture(PAGE));
    assert_eq!(
        report.csv_content(),
        "kind,value\nsynthetic,1\nsynthetic,2\n"
    );
    assert_eq!(report.reported_entries(), 2);
    assert!(report.has_more());
    let cursor = report.continuation().expect("next cursor");
    assert_eq!(cursor.sequence(), 7);
    assert_eq!(
        cursor.account(),
        Some(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").expect("test principal"))
    );

    let empty = page(&fixture(EMPTY));
    assert_eq!(empty.csv_content(), "");
    assert_eq!(empty.reported_entries(), 0);
    assert!(!empty.has_more());
    assert_eq!(empty.continuation(), None);

    let unscoped = page(&fixture(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/caffeine-audit/optional-account.hex"
    ))));
    assert_eq!(unscoped.csv_content(), "header\n");
    assert_eq!(unscoped.reported_entries(), 0);
    assert!(unscoped.has_more());
    let cursor = unscoped.continuation().expect("reported cursor");
    assert_eq!(cursor.account(), None);
    assert_eq!(cursor.sequence(), u64::MAX);
}

#[test]
fn advertised_errors_remain_distinct_from_empty_history() {
    for (hex, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-audit/unauthorized.hex"
            )),
            AuditLogProviderError::NotAuthorized(
                Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").expect("test principal"),
            ),
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-audit/invalid-request.hex"
            )),
            AuditLogProviderError::InvalidRequest,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-audit/internal.hex"
            )),
            AuditLogProviderError::InternalError,
        ),
    ] {
        assert_eq!(
            decode_audit_log_reply(&fixture(hex), limits()),
            Ok(AuditLogReply::ProviderFailure(expected))
        );
    }
}

#[test]
fn page_bounds_and_missing_continuation_fail_without_partial_output() {
    let bytes = fixture(PAGE);
    let mut bounded = limits();
    bounded.max_bytes = positive(bytes.len());
    bounded.max_csv_bytes = positive(page(&bytes).csv_content().len());
    bounded.max_reported_entries = NonZeroU64::new(2).expect("positive limit");
    assert!(decode_audit_log_reply(&bytes, bounded).is_ok());
    let exact = bounded;
    bounded.max_bytes = positive(bytes.len() - 1);
    assert_eq!(
        decode_audit_log_reply(&bytes, bounded),
        Err(AuditLogReplyError::ReplyTooLarge)
    );
    bounded = exact;
    bounded.max_csv_bytes = positive(exact.max_csv_bytes.get() - 1);
    assert_eq!(
        decode_audit_log_reply(&bytes, bounded),
        Err(AuditLogReplyError::CsvTooLarge)
    );
    bounded = exact;
    bounded.max_reported_entries = NonZeroU64::new(1).expect("positive limit");
    assert_eq!(
        decode_audit_log_reply(&bytes, bounded),
        Err(AuditLogReplyError::TooManyReportedEntries)
    );
    assert_eq!(
        decode_audit_log_reply(
            &fixture(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-audit/missing-continuation.hex"
            ))),
            limits()
        ),
        Err(AuditLogReplyError::MissingContinuation)
    );
}

#[test]
fn opaque_text_is_not_normalized_or_counted_as_verified_rows() {
    let text = "\"é\",\"quoted\r\ntext\"\r\n=1+1,arbitrary";
    let response: wire::AuditLogDownloadResult = Ok(wire::AuditLogDownloadResponse {
        csv_content: text.to_owned(),
        continuation_token: None,
        num_returned_entries: u64::MAX,
        has_more: false,
    });
    let bytes = candid::encode_one(response).expect("synthetic response");
    let mut bounded = limits();
    assert_eq!(
        decode_audit_log_reply(&bytes, bounded),
        Err(AuditLogReplyError::TooManyReportedEntries)
    );
    bounded.max_reported_entries = NonZeroU64::MAX;
    bounded.max_csv_bytes = positive(text.len());
    let AuditLogReply::ReportedPage(report) =
        decode_audit_log_reply(&bytes, bounded).expect("bounded report")
    else {
        panic!("expected page")
    };
    assert_eq!(report.csv_content(), text);
    assert_eq!(report.reported_entries(), u64::MAX);
    bounded.max_csv_bytes = positive(text.chars().count());
    assert_eq!(
        decode_audit_log_reply(&bytes, bounded),
        Err(AuditLogReplyError::CsvTooLarge)
    );
}

#[test]
fn malformed_unknown_and_over_budget_candid_never_become_an_empty_page() {
    #[derive(CandidType)]
    enum UnknownError {
        Unsupported,
    }
    let unknown: Result<(), UnknownError> = Err(UnknownError::Unsupported);
    let valid = fixture(PAGE);
    for bytes in [
        Vec::new(),
        candid::encode_args(()).expect("empty tuple"),
        valid[..valid.len() - 1].to_vec(),
        candid::encode_one(unknown).expect("unknown error"),
    ] {
        assert_eq!(
            decode_audit_log_reply(&bytes, limits()),
            Err(AuditLogReplyError::InvalidReply)
        );
    }
    let mut bounded = limits();
    bounded.decoding_quota = positive(1);
    assert_eq!(
        decode_audit_log_reply(&valid, bounded),
        Err(AuditLogReplyError::InvalidReply)
    );
    bounded = limits();
    bounded.max_type_entries = positive(1);
    assert_eq!(
        decode_audit_log_reply(&valid, bounded),
        Err(AuditLogReplyError::InvalidReply)
    );

    let result: wire::AuditLogDownloadResult =
        Err(wire::AuditLogError::InvalidRequest("diagnostic".to_owned()));
    let extended = candid::encode_args((result, "additional diagnostic".repeat(4)))
        .expect("additional argument");
    assert_eq!(
        decode_audit_log_reply(&extended, limits()),
        Ok(AuditLogReply::ProviderFailure(
            AuditLogProviderError::InvalidRequest
        ))
    );
    bounded = limits();
    bounded.skipping_quota = positive(1);
    assert_eq!(
        decode_audit_log_reply(&extended, bounded),
        Err(AuditLogReplyError::InvalidReply)
    );
}
