//! Monotonic local read invalidation; never an installation freshness authority.
use candid::{CandidType, Deserialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct GatewayReadGeneration {
    sequence: u64,
    exhausted: bool,
}
impl GatewayReadGeneration {
    pub(crate) const fn new() -> Self {
        Self {
            sequence: 0,
            exhausted: false,
        }
    }
    pub(crate) const fn current(self) -> Option<u64> {
        if self.exhausted {
            None
        } else {
            Some(self.sequence)
        }
    }
    pub(crate) fn invalidate(&mut self) {
        if let Some(next) = self.sequence.checked_add(1) {
            self.sequence = next;
        } else {
            // Revocation must remain possible; exhaustion disables all new reads.
            self.exhausted = true;
        }
    }
    pub(crate) const fn valid(self) -> bool {
        !self.exhausted || self.sequence == u64::MAX
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exhaustion_never_wraps_or_revalidates_old_reads() {
        let mut generation = GatewayReadGeneration {
            sequence: u64::MAX - 1,
            exhausted: false,
        };
        generation.invalidate();
        assert_eq!(generation.current(), Some(u64::MAX));
        generation.invalidate();
        assert_eq!(generation.current(), None);
        generation.invalidate();
        assert_eq!(generation.current(), None);
        assert!(generation.valid());
        assert!(
            !GatewayReadGeneration {
                sequence: 1,
                exhausted: true
            }
            .valid()
        );
    }
}
