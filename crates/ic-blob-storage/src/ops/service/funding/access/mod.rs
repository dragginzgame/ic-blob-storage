//! Host-owned synchronous access; no journal borrow may survive an IC await.
use super::{Memory, StableFundingJournal};

/// Access the same installed journal before dispatch and in its callback.
/// Implementations must invoke the closure exactly once, synchronously, without
/// catching traps, changing bindings or substituting a different journal. They
/// must release the borrow before returning. This trait acquires no host evidence.
pub trait FundingJournalAccess {
    /// Exclusively granted stable-memory implementation.
    type Memory: Memory;
    /// Apply one synchronous operation to the host's journal.
    fn with_funding_journal<R>(
        &self,
        operation: impl FnOnce(&mut StableFundingJournal<Self::Memory>) -> R,
    ) -> R;
}
