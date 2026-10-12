# Canic service correctness review

Date: 2026-10-12. Blob is released 0.22.13 at
`2657177a6e089d66aa7371c5f212e9849a15a8a2`. Canic's released base is 0.111.0
`25b73cd403b20b4e148c6e340f8c760a26d97f6e`; its wider integration source is dirty.
This review reads Canic without editing or building it. Pending compatible Blob
0.22.14 clarifies an existing host boundary, with no runtime/schema change.

## Confirmed decoder gap

[Canic #524](https://github.com/dragginzgame/canic/issues/524) owns the correction.
Its Blob lifecycle CONFIGURATION_LIMITS inherits a skipping quota of 1,024 from
ordinary requests. Both dedicated and embedded application configuration use it.
The committed 0.111.0 lifecycle source has the same setting. ArgumentLimits
delegates directly to Candid's configured argument decoder.

An isolated native probe with actual current Blob DTOs and Candid 0.10.38 proves
that quota 1,024 accepts a retired trusted_uploader field and a small extra
application-init argument, yielding the exact current DTO. The nested embedded
Blob record behaves the same. Quota zero accepts both current shapes and rejects
all four variants. Dedicated input sizes are 545/582/552 bytes for current,
retired-field and extra-argument payloads; embedded sizes are 567/604/574. Existing
byte/work/type/header bounds remain in the probe. All inputs fit the byte bound.

This is codec execution and reviewed adapter wiring, not actual Canic installation
or a demonstrated per-upload authority bypass. The smallest prepared owner patch
sets only CONFIGURATION_LIMITS.skipping_quota to zero. Git patch applicability
checking passes; the patch is unapplied because Canic remains read-only here.
Canic must qualify current dedicated/embedded installation, rejected old fields
and extra arguments, and failed-reinstall rollback preserving application/Blob
state and exact bindings. Framework envelope and ordinary endpoint request limits
remain independent. No copied decoder or downstream fixture is introduced in Blob.
Blob's passive DTO Rustdoc and service/host guidance now state this requirement;
candidate validation cannot detect discarded fields after typed decoding.

## Other reviewed boundaries

The adapter delegates through the shared endpoint owner with lexical attributes,
bounded requests and actual caller/service context captured before continuity
awaits. Service borrowing observes platform versions; account/gateway workflows
recheck through the existing access owners. Memory opens host grants without
another runtime. Installation validates actual identity before opening service
grants; restoration is synchronous and fenced. The application retains its sole
lifecycle and sampler. No additional source-level gap was established in those
paths. Installed acceptance stays bound to [earlier artifacts](shared083-canic02212.md).

Only passive funding inspection is mounted. Guarded dispatch, credit acquisition
and renewal remain [Canic #494](https://github.com/dragginzgame/canic/issues/494)
after authenticated provider evidence under [Blob #34](https://github.com/dragginzgame/ic-blob-storage/issues/34).
Parent post-allocation initialization remains controller-only under
[Canic #493](https://github.com/dragginzgame/canic/issues/493). Exact decoding is
separate from those authorities and real verifier/provider/browser/application
acceptance. Existing successful composition journeys do not close the decoder gap.

## Evidence and scope

Raw reviewed source hashes, unchanged-source verification, native probe source,
manifest/independent lock, Candid payloads/hashes, successful output and prepared
patch remain at `.tmp/review-validation/canic-service-audit.2JWFRZ/`. The initial
compile omitted Candid's value feature; its failure remains alongside the
corrected successful attempt. The probe uses Blob's build directory and passive
contracts without resolving or changing incoming root dependency edits.

Documentation links, formatting and diff checks pass. Root Cargo files, receipt,
index and refs remain unchanged by this work. No full CI/release gate, fresh
PocketIC/Canic execution, sibling edit, deployment, provider request, paid effect
or cleanup is performed. Blob #61 remains closed for its delivered endpoint
declaration owner; #524 owns this Canic decoder repair.

## Explicitly authorized follow-up

The maintainer subsequently requested applying the fix in Canic. Its existing
CONFIGURATION_LIMITS now sets skipping_quota to zero, and two native regression
cases join the already classified fixture_payloads target. They exercise current
dedicated/embedded inputs, unknown fields, extra arguments and preserved ordinary
request skipping. Formatting, inventory and scoped documentation checks pass.
Canic package/dependency inputs and the index remain unchanged.

The actual focused Canic test command stops before running these cases in five
existing denied Auth dead-code errors in canic-core. No Auth source or lint policy
is changed to bypass that failure. Source/lock inputs, the failed attempt and
scoped repair evidence stay with Canic under `.tmp/issue-524/`. Installed
rejection/rollback and #524 closure remain pending. The earlier codec results
above retain their independent native probe identity and are not relabelled as
passing Canic tests or PocketIC execution.
