//! Explicit native fixture custody; corruption tests retain backing bytes.
//! One runtime is retained across openings. Drop each collection before reopening;
//! the raw backing never grants a second allocation lease.
use ic_memory::{
    Memory, MemoryManagerConfig, MemoryRuntime, RuntimeMemory, VectorMemory,
    testing::fixture_runtime,
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Clone)]
pub(crate) struct FixtureBackingMemory {
    bytes: VectorMemory,
    runtime: Option<Rc<MemoryRuntime<Self>>>,
}
impl FixtureBackingMemory {
    pub(crate) fn lease(&self) -> RuntimeMemory<Self> {
        self.runtime
            .as_ref()
            .expect("fixture owner")
            .open_memory("fixture.store.v1")
            .unwrap()
    }
    pub(crate) fn snapshot(&self) -> Self {
        Self::from_backing(Rc::new(RefCell::new(self.bytes.borrow().clone())))
    }
    fn from_backing(bytes: VectorMemory) -> Self {
        // The manager's backing has no runtime reference, avoiding a custody cycle.
        let backing = Self {
            bytes: bytes.clone(),
            runtime: None,
        };
        let runtime = fixture_runtime(
            backing,
            "fixture",
            "fixture.",
            &["fixture.store.v1"],
            MemoryManagerConfig::new(1).unwrap(),
        )
        .unwrap();
        Self {
            bytes,
            runtime: Some(Rc::new(runtime)),
        }
    }
    pub(crate) fn borrow(&self) -> Ref<'_, Vec<u8>> {
        self.bytes.borrow()
    }
}
impl Default for FixtureBackingMemory {
    fn default() -> Self {
        Self::from_backing(VectorMemory::default())
    }
}
impl Memory for FixtureBackingMemory {
    fn size(&self) -> u64 {
        self.bytes.size()
    }
    fn grow(&self, pages: u64) -> i64 {
        self.bytes.grow(pages)
    }
    fn read(&self, offset: u64, bytes: &mut [u8]) {
        self.bytes.read(offset, bytes);
    }
    fn write(&self, offset: u64, bytes: &[u8]) {
        self.bytes.write(offset, bytes);
    }
}
