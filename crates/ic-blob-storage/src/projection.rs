//! Schema companions for actual mounted Rust types. Wire owns container and DTO layout.

use ic_platform_types::Principal;
use ic_wire::typescript::{GenerationError, SchemaContext, Wire};
use serde_json::{Value, json};
use std::{
    marker::PhantomData,
    num::{NonZeroU64, NonZeroU128, NonZeroUsize},
};

pub(crate) trait Project {
    type Schema: Wire;
}

pub(crate) struct Shape<T>(PhantomData<T>);

macro_rules! native {
    ($($ty:ty),+) => { $(impl Project for $ty { type Schema = Self; })+ };
}
native!(
    (),
    bool,
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    String,
    Principal
);

macro_rules! bounded {
    ($ty:ty, $shape:expr) => {
        impl Project for $ty {
            type Schema = Shape<Self>;
        }
        impl Wire for Shape<$ty> {
            fn schema(_: &mut SchemaContext) -> Result<Value, GenerationError> {
                Ok($shape)
            }
        }
    };
}

bounded!(NonZeroU128, json!({"k":"u","bits":128,"min":1}));
bounded!(NonZeroU64, json!({"k":"u","bits":64,"min":1}));
// Serde encodes usize as u64. The release Wasm accepts at most u32::MAX.
bounded!(usize, json!({"k":"u","bits":64,"max":u32::MAX}));
bounded!(
    NonZeroUsize,
    json!({"k":"u","bits":64,"min":1,"max":u32::MAX})
);

impl<T: Project> Project for Vec<T> {
    type Schema = Vec<T::Schema>;
}
impl<T: Project> Project for Option<T> {
    type Schema = Option<T::Schema>;
}
impl<T: Project, E: Project> Project for Result<T, E> {
    type Schema = Result<T::Schema, E::Schema>;
}
impl<T: Project, U: Project> Project for (T, U) {
    type Schema = (T::Schema, U::Schema);
}
impl<const N: usize> Project for [u8; N]
where
    [u8; N]: Wire,
{
    type Schema = Self;
}
