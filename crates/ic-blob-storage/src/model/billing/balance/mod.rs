//! Bounded balance amounts, without provider identity or freshness guarantees.

/// Unsigned cycle amounts observed together at a billing boundary.
///
/// Each value fits `u128`. The reported total is preserved independently of the
/// components: this type does not invent an aggregation rule or prove funding
/// completion, spendability, account binding or freshness. It is not a persisted
/// record or provider wire schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BalanceAmounts {
    total: u128,
    prepaid: u128,
    promotional: u128,
    ledger: u128,
}

impl BalanceAmounts {
    /// Collect already bounded amounts without recomputing the reported total.
    #[must_use]
    pub const fn new(total: u128, prepaid: u128, promotional: u128, ledger: u128) -> Self {
        Self {
            total,
            prepaid,
            promotional,
            ledger,
        }
    }

    /// Independently reported total, not a sum calculated by this library.
    #[must_use]
    pub const fn total(self) -> u128 {
        self.total
    }

    /// Reported prepaid cycles.
    #[must_use]
    pub const fn prepaid(self) -> u128 {
        self.prepaid
    }

    /// Reported promotional cycles.
    #[must_use]
    pub const fn promotional(self) -> u128 {
        self.promotional
    }

    /// Reported ledger cycles.
    #[must_use]
    pub const fn ledger(self) -> u128 {
        self.ledger
    }
}
