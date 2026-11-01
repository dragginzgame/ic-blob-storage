//! Passive descriptor recovery from the single tenant-owned admission journal.

/// Original validated hash metadata; owning a copy grants no authority.
///
/// The admission owner retains at most `max_headers` entries and
/// `max_header_bytes` framed UTF-8 bytes per lifetime object slot. Descriptors
/// expose owned read-only copies. Header names/case/values are not rewritten
/// for HTTP use.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentHeader {
    /// Exact name from the first successful manifest preparation.
    pub name: String,
    /// Exact value from the first successful manifest preparation.
    pub value: String,
}
