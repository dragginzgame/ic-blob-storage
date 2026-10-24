//! Canonical method names generated from the declarations mounted by hosts.
macro_rules! names {
    ($emit:ident; $([$constant:ident, $id:tt; $($entry:tt)*])*) => {
        $(
            /// Canonical declared method; linking the constant exports no endpoint.
            pub const $constant: &str = crate::__blob_method_spec!($($entry)*).name;
        )*
    };
}
crate::__blob_service_catalog!(
    dispatch = [names],
    emit = unused,
    workflow = unused,
    context = unused,
    contracts = crate,
    clock = unused,
    caller = unused,
    trap = unused,
);
