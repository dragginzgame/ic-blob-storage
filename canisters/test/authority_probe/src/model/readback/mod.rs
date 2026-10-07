//! One bounded in-flight local read, invalidated without freeing it early.
pub(crate) struct ReadSlot {
    last: u64,
    pending: Option<(u64, bool)>,
}

impl ReadSlot {
    pub(crate) const fn new() -> Self {
        Self {
            last: 0,
            pending: None,
        }
    }

    pub(crate) fn busy(&self) -> bool {
        self.pending.is_some()
    }

    pub(crate) fn observation(&self) -> (u64, Option<(u64, bool)>) {
        (self.last, self.pending)
    }

    pub(crate) fn begin(&mut self) -> Option<u64> {
        if self.busy() {
            return None;
        }
        self.last = self.last.checked_add(1)?;
        self.pending = Some((self.last, true));
        Some(self.last)
    }

    pub(crate) fn invalidate(&mut self) {
        if let Some((_, valid)) = &mut self.pending {
            *valid = false;
        }
    }

    /// Only the exact callback releases the slot, even after invalidation.
    pub(crate) fn finish(&mut self, token: u64) -> bool {
        if let Some((pending, valid)) = self.pending
            && pending == token
        {
            self.pending = None;
            return valid;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::ReadSlot;

    #[test]
    fn invalidation_retains_capacity_and_old_callbacks_cannot_free_a_new_read() {
        let mut slot = ReadSlot::new();
        let old = slot.begin().expect("first read");
        slot.invalidate();
        assert!(slot.busy());
        assert_eq!(slot.begin(), None);
        assert!(!slot.finish(old));
        let new = slot.begin().expect("old callback freed its slot");
        assert_ne!(old, new);
        assert!(!slot.finish(old));
        assert!(slot.busy());
        assert!(slot.finish(new));
        assert!(!slot.busy());
    }
}
