# ic-blob-storage-wire

Shared sparse IC Wire frames and RPC limits for Blob Storage hosts and callers.
The embedding host mounts endpoints, captures authorization context, and traps
on framing or encoding failure.

Mount `Update`, `Query` and `Management` through the `Contract` trait. A
contract pairs its request and response types with its routing ID, directional
limits and codecs. A host may choose a different exported name while retaining
that routing ID.

Enable `inspection` for native schema projections. Canic can register
`RequestSchema<Update>` and `ResponseSchema<Update>` for the corresponding
contract body types, then supply its outer frame schema. A standalone generator
can register `ic_wire::Message<RequestSchema<Update>>` for the complete request.
