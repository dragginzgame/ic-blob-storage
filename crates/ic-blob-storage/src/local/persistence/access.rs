//! A participant grant projected onto Storage's host or canister scope API.
//! Storage exclusively owns allocation, native admission and close/reopen.
use ic_storage::retained::{Error, Grant, Read, Write};
#[cfg(not(target_arch = "wasm32"))]
use {ic_storage::retained::Runtime, std::rc::Rc};

/// Access to one host-selected root through a supplier-issued participant grant.
/// Never clone a grant or construct a second runtime over the same native memory.
pub struct Access {
    #[cfg(not(target_arch = "wasm32"))]
    runtime: Rc<Runtime>,
    grant: Grant,
    root: u64,
}
impl Access {
    /// Bind a native test/application runtime and its existing participant grant.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub const fn host(runtime: Rc<Runtime>, grant: Grant, root: u64) -> Self {
        Self {
            runtime,
            grant,
            root,
        }
    }
    /// Bind the host's one prepared canister runtime and an existing grant.
    #[cfg(target_arch = "wasm32")]
    #[must_use]
    pub const fn canister(grant: Grant, root: u64) -> Self {
        Self { grant, root }
    }
    /// Read granted native records synchronously; borrowed values cannot escape.
    /// # Errors
    /// Returns Storage's stale/foreign grant, lifecycle or poison refusal.
    pub fn read<R>(&self, operation: impl for<'a> FnOnce(Read<'a>, u64) -> R) -> Result<R, Error> {
        #[cfg(not(target_arch = "wasm32"))]
        return self
            .runtime
            .read(&[&self.grant], |view| operation(view, self.root));
        #[cfg(target_arch = "wasm32")]
        ic_storage::retained::Canister::read(&[&self.grant], |view| operation(view, self.root))
    }
    /// Mutate within Storage's poison scope and the enclosing IC transaction.
    /// Ordinary returned errors do not undo preceding writes.
    /// # Errors
    /// Returns Storage's stale/foreign grant, lifecycle or poison refusal.
    pub fn update<R>(
        &self,
        operation: impl for<'a> FnOnce(Write<'a>, u64) -> R,
    ) -> Result<R, Error> {
        #[cfg(not(target_arch = "wasm32"))]
        return self
            .runtime
            .update(&[&self.grant], |view| operation(view, self.root));
        #[cfg(target_arch = "wasm32")]
        ic_storage::retained::Canister::update(&[&self.grant], |view| operation(view, self.root))
    }
}
