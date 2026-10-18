//! Explicit native fixture custody; corruption tests retain backing bytes.
//! One runtime is retained across openings. Drop each collection before reopening;
//! the raw backing never grants a second allocation lease.
//! This owner is not itself backing memory; collections use `VectorMemory`.
use ic_memory::{
    MemoryManagerConfig, MemoryRuntime, RuntimeMemory, VectorMemory, testing::fixture_runtime,
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Clone)]
pub(crate) struct FixtureMemory {
    bytes: VectorMemory,
    runtime: Rc<MemoryRuntime<VectorMemory>>,
}
impl FixtureMemory {
    pub(crate) fn lease(&self) -> RuntimeMemory<VectorMemory> {
        self.runtime.open_memory("fixture.store.v1").unwrap()
    }
    pub(crate) fn snapshot(&self) -> Self {
        Self::from_backing(Rc::new(RefCell::new(self.bytes.borrow().clone())))
    }
    fn from_backing(bytes: VectorMemory) -> Self {
        let runtime = fixture_runtime(
            bytes.clone(),
            "fixture",
            "fixture.",
            &["fixture.store.v1"],
            MemoryManagerConfig::new(1).unwrap(),
        )
        .unwrap();
        Self {
            bytes,
            runtime: Rc::new(runtime),
        }
    }
    pub(crate) fn borrow(&self) -> Ref<'_, Vec<u8>> {
        self.bytes.borrow()
    }
}
impl Default for FixtureMemory {
    fn default() -> Self {
        Self::from_backing(VectorMemory::default())
    }
}
