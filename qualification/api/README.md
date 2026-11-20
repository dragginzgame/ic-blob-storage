# Callable Wire methods

Run `make check-callable-api` to build the standalone and reference-app release
Wasms and compare their callable names and modes. The baseline comes from both
artifacts rebuilt from tag `v0.107.1` with its Cargo.lock. To review an intended
change, inspect the comparison runner's candidate directory and the corresponding
mount. Keep baseline edits explicit.

All eight callable methods use Wire. IC lifecycle hooks have no callable method
name. The Caffeine callback and enrollment probes belong to private qualification.
The installed endpoint suite also checks a request and reply captured with the
0.107.1 codecs.

A full per-method schema baseline requires typed mount descriptors for request,
reply and limits. Wire 0.27.1's schema trait lacks projections for Blob's
`NonZeroU128` reference and upload IDs, `NonZeroU64`, `NonZeroUsize` and
`usize` fields. When these are available, generate manifests from the mounted
DTOs and compare them alongside this inventory.
