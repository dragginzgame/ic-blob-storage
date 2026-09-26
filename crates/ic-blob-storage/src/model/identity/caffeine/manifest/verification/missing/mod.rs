//! Bounded current views of unverified chunk positions, without reserving work.

use std::num::NonZeroUsize;

use super::super::{CaffeineChunkRange, CaffeineManifestError};
use super::CaffeineChunkVerifier;

#[cfg(test)]
mod tests;

/// Independent processing and result-allocation budgets for a missing-chunk page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissingChunkPageLimits {
    /// Maximum positions inspected, including already verified positions.
    pub max_scan: NonZeroUsize,
    /// Maximum missing ranges returned; repeated calls allocate no retained history.
    pub max_results: NonZeroUsize,
}

/// Ascending unverified ranges observed during one bounded scan.
///
/// This is a current view, not a snapshot, download request or in-flight reservation.
/// Another reader can select the same range. Always verify bytes on arrival, even
/// if progress advanced after the page was read. These indices belong to the
/// verifier's manifest; they grant no provider or tenant authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingChunkPage {
    /// Exact local byte locations of unverified leaves in increasing index order.
    pub chunks: Vec<CaffeineChunkRange>,
    /// First uninspected position, including after an empty filtered page.
    /// None means the scan reached the end, not that all positions passed verification.
    pub next_index: Option<u64>,
    /// Number of positions inspected, including those already verified.
    pub scanned: usize,
}

impl CaffeineChunkVerifier {
    /// Inspect a bounded page of still-unverified positions at or after `start_index`.
    ///
    /// Both budgets apply independently: even a page with no results can have a
    /// continuation. Follow it to finish the scan. Selecting a range never marks
    /// it verified or reserves it; only successful byte verification changes coverage.
    /// Pages observe current state, so a position verified before a later call is
    /// skipped. Repeating a page without changing coverage returns the same view.
    ///
    /// The position is not a durable checkpoint. Start at zero for each new verifier
    /// or retry sweep; reuse of a prior scan's end cannot prove work was completed.
    /// # Errors
    /// Rejects a start beyond the manifest's chunk count before allocation or scan.
    /// Exactly the chunk count is a valid end position and returns an empty page.
    pub fn missing_chunks(
        &self,
        start_index: u64,
        limits: MissingChunkPageLimits,
    ) -> Result<MissingChunkPage, CaffeineManifestError> {
        let invalid = CaffeineManifestError::ChunkIndexOutOfRange { index: start_index };
        let start = usize::try_from(start_index).map_err(|_| invalid)?;
        let count = self.manifest().chunk_count();
        if start > count {
            return Err(invalid);
        }
        let mut page = MissingChunkPage {
            chunks: Vec::new(),
            next_index: None,
            scanned: 0,
        };
        // The manifest checked that its chunk count fits u64. Incrementing this
        // index at most to that count is safe even on narrower usize targets.
        let mut next = start_index;
        for _ in (start..count).take(limits.max_scan.get()) {
            page.scanned += 1;
            if !self.is_verified(next)? {
                page.chunks.push(self.manifest().chunk_range(next)?);
            }
            next += 1;
            if page.chunks.len() == limits.max_results.get() {
                break;
            }
        }
        if start + page.scanned < count {
            page.next_index = Some(next);
        }
        Ok(page)
    }
}
