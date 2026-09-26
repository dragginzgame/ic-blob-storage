# 📦 Introducing ic-blob-storage: a storage service foundation for Internet Computer apps

I wanted to share an early introduction to **ic-blob-storage**, an open-source Rust project building toward an independent blob-storage service for Internet Computer canisters.

The intended developer experience is straightforward: an application uploads an object, keeps references to it, reads and verifies its contents, and releases it when it is no longer needed. The service handles the shared responsibilities around authorization, quotas, provider access, billing, and deletion.

The project is currently at **0.1.7**. There is an implemented and tested Rust core, but the complete canister service is still ahead. Persistent workflows, provider communication, upload/read clients, and deployment adapters have not been implemented yet.

This is a good point to introduce the direction and get feedback from people building applications that need it.

🔗 **Repository:** https://github.com/dragginzgame/ic-blob-storage  
📄 **License:** MIT

## Why build this?

Storing a file involves more than getting its bytes to a storage provider.

An application also needs answers to questions like:

- Which tenant owns this object, and who can release it?
- What happens when several application records reference the same object?
- Can an interrupted request be retried safely?
- When does deleting a reference actually free storage?
- When does billing stop?
- What happens if a canister restores an older backup?

These responsibilities are easy to spread across application code, deployment tooling, and provider-specific helpers. The aim here is to give them a clear home, with one implementation that applications can build on.

The work grows out of blob-storage capabilities in **Canic**, while reviewing the design independently. Existing capabilities provide a useful starting point; the new service needs its own ownership rules, recovery behavior, and evidence.

## 🧱 Where we are today

The current library implements the foundations for reasoning about objects, references, access, and resource usage.

| Area | Current state |
|---|---|
| Content identity and verification | Raw-byte SHA-256 verification, including incremental verification without buffering the complete object |
| Object and reference lifecycle | Local models for retaining references, releasing them, tracking deletion, and recording settlement |
| Tenant policy | Explicit ownership checks and bounded reference-status reads |
| Resource limits | A bounded in-memory catalog with tenant quotas and separate physical-storage and unsettled billing-byte accounting |
| Request retries | Exact local request receipts that preserve previous results and reject conflicting reuse |
| Gateway and provider replies | Local membership rules, stale-response rejection, and bounded decoding of selected Caffeine replies |
| Persistent service and provider execution | Still to come |
| Clients and deployment adapters | Still to come |

**These are local library capabilities. They do not yet provide an end-to-end storage service.**

[details="What changed in 0.1.7?"]

The latest release brings several of those pieces together in a bounded, in-memory catalog.

It tracks confirmed objects, their references, their ownership bindings, and receipts for reference operations. Limits cover both byte usage and metadata: objects, references, and retained request results all consume capacity.

That distinction matters. A zero-byte object can still consume bookkeeping space, and an object whose storage bill is settled can still have history that must be retained.

The release also adds tenant-checked reads across multiple objects. A caller cannot use a mixed batch to obtain partial information about another tenant’s objects.

Request-result lookup is separate from current reference status. If a reference was successfully retained and later released, looking up the original request still returns its original result. It does not make that reference active again.

Deletion enumeration is bounded too: pages limit both how many objects are scanned and how many results are returned, with gateway authorization checked on each page.

The catalog currently starts with objects treated as already confirmed. Upload reservations, durable state, and real provider effects remain future work.

[/details]

## 🔄 The lifecycle matters

One central design choice is to keep three events separate:

**Releasing an application reference → confirming provider deletion → confirming billing settlement**

Those events can happen at different times.

For example, imagine an object used by two records in an application. Releasing one reference should leave the other intact. Releasing the final reference can queue deletion, but the provider may still hold the bytes. Even after physical deletion, financial obligations may need separate confirmation.

The local model represents these stages explicitly.

[details="Why separate references, stored bytes, and billing?"]

Each stage answers a different question:

- **References:** Does the application still need this object?
- **Physical storage:** Is the provider still holding its bytes?
- **Billing:** Has the remaining financial obligation been resolved?

Freeing a tenant’s logical quota cannot make still-stored bytes disappear from global accounting.

Likewise, a successful balance query cannot prove that a particular payment completed, and an upload progress reply cannot by itself prove durable storage.

Keeping these facts separate helps prevent optimistic bookkeeping from becoming a correctness problem.

The current implementation tracks byte-based obligations. A complete monetary ledger and authoritative provider evidence are still required for the service.

[/details]

## 🧭 Where it is going

The target is a complete application journey:

> An authorized tenant uploads a bounded object, resumes after interruption, reads and verifies the bytes, and releases its reference through confirmed deletion and billing cessation.

That journey should work through two deployment options:

- **Standalone:** a service canister that does not require Canic.
- **Canic-managed:** an adapter that uses Canic’s generic deployment and lifecycle facilities.

Both are planned to use the same service handlers, blob API, and tenant rules. The core already builds without Canic.

The intended deliverables also include upload/read clients and operator tooling for readiness checks, gateway administration, funding, and diagnostics.

[details="The next implementation milestones"]

### 1. Resolve the provider and consumer contract

Before implementing provider effects, we need firm answers about account and namespace ownership, supported onboarding, upload completion, uncertain payments, deletion, and recovery.

We also need a concrete consumer journey and numeric resource limits. Application behavior should help determine the contract.

### 2. Add durable workflows

The service needs to record intent and reserve capacity before starting external effects.

Interrupted operations must retain their identity, accounting, and unresolved outcome. Restoring an older backup must not silently permit reused identities or repeated payments.

### 3. Build endpoints, clients, and both adapters

Authentication and delegation belong at the endpoints. Storage policy and workflows belong in the shared service.

The standalone and Canic-managed deployments should exercise the same behavior, with operator tooling using that service API too.

### 4. Demonstrate the full journey

Qualification needs canister lifecycle and interruption tests using PocketIC, alongside separate evidence from the actual provider.

Tests using a substitute provider are useful, but they cannot establish how a deployed provider handles lost replies, retention, or billing.

### 5. Complete the Canic extraction

Existing blob capabilities in Canic need working replacements here before removal there.

Retiring existing installations is a separate responsibility: removing code does not settle outstanding balances, stored objects, or uncertain operations.

[/details]

## ☕ What about the storage provider?

**Caffeine is the current integration candidate.** The repository includes a reviewed integration baseline and local decoding and validation for selected provider replies.

Provider qualification remains open. In particular, the project still needs a supported independent onboarding arrangement and reliable evidence for completion, recovery, and settlement.

[details="The provider questions we still need to close"]

The important questions are concrete:

- **Ownership:** Which account, project, or namespace belongs to the service, and who controls its callbacks?
- **Upload completion:** After a lost response, how can we determine whether the original upload completed?
- **Payments:** After an uncertain top-up, how can we establish the accepted or refunded amount for that exact operation?
- **Deletion:** What evidence ties a deletion confirmation to the correct object and operation?
- **Recovery:** Which authoritative records survive the supported backup and restore boundary?
- **Billing cessation:** What proves that an object’s remaining charges have ended?

A client library or a successfully decoded response does not answer all of these questions. They need to be resolved before the service can claim reliable recovery behavior.

[/details]

## 🔍 Some deliberate boundaries

The initial scope is a bounded storage journey with clear ownership and recovery rules.

Shared cross-tenant deduplication, generic provider plugins, and new confidentiality guarantees are deferred. Serving and access behavior will need their own explicit contract; possession of a content hash or a download URL should not be confused with tenant authority or a privacy guarantee.

The APIs are also pre-1.0 and can change.

[details="One design question worth calling out: repeated content"]

The current local model conservatively prevents a provider root from being reassigned to a different object, including after settlement.

This protects against delayed deletion confirmations being applied to a newer object with the same root. It also restricts repeated uploads of identical content as separate objects.

That is a current safety restriction, not a settled product requirement.

Repeated content, multiple references, delete-and-reupload behavior, and provider-supported operation identity all need to be considered together before freezing the production design.

[/details]

## 💬 Feedback welcome

I would especially like to hear from developers with concrete storage workflows:

- What kinds of objects are you storing, and how frequently do they change?
- Do several records or canisters need to reference the same object?
- Would you use a standalone service, a Canic-managed deployment, or both?
- What upload interruption and recovery cases matter most to your application?
- Do you need public serving, restricted access, or both?

Experience with independent Caffeine integrations would also be useful, particularly around onboarding and recovery semantics.

The foundations are taking shape. The next step is turning them into a complete, demonstrated storage journey that applications can depend on.

**Code and design notes:** https://github.com/dragginzgame/ic-blob-storage
