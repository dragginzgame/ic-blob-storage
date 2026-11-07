SHELL := /bin/bash
.DEFAULT_GOAL := help

# Keep all builds in this repository, including calls from another workspace.
export CARGO_TARGET_DIR := $(CURDIR)/target
IC_TOOLS := wasm-opt
LOCAL_TOOL_INSTALL_TARGETS := install-ic-tools install-testkit install-release-editor
LOCAL_TOOL_CHECK_TARGETS := ic-tools-check testkit-check release-editor-check
export BLOB_PREPARED_TOOLS_ROOT ?= $(CURDIR)
RELEASE_EDITOR_ARGS := --consumer "$(BLOB_PREPARED_TOOLS_ROOT)" --package ic-build --lockfile "$(CURDIR)/Cargo.lock" --bin ic-build-release-metadata --profile debug --features cli
override SHARED_TOOLING_ROOT := $(CURDIR)
override RELEASE_DELIVERY_POLICY := ci/release-delivery-policy.txt
export PATH := $(CURDIR)/.tools/ic/bin:$(PATH)
include make/consumer.mk
# Shared's outer batch commands need ordinary context. Captured child Make
# invocations select their own artifact/tool roots; never inherit this checkout's.
ifneq ($(RELEASE_BATCH_ACTIVE),1)
_blob_frozen_goals := $(filter release-batch-%,$(MAKECMDGOALS))
ifneq ($(strip $(RELEASE_GROUPS)),)
_blob_frozen_goals += $(filter release-patch release-minor release-major,$(MAKECMDGOALS))
endif
ifneq ($(strip $(_blob_frozen_goals)),)
unexport CARGO_TARGET_DIR BLOB_PREPARED_TOOLS_ROOT
endif
endif
export BLOB_AUTHORITY_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_authority_probe.wasm
export BLOB_ADMISSION_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_admission_probe.wasm
export BLOB_STORAGE_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_storage_probe.wasm
export BLOB_CONSUMER_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_consumer_probe.wasm
export BLOB_GATEWAY_SOURCE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_gateway_source.wasm
export BLOB_FUNDING_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_funding_probe.wasm
export BLOB_STANDALONE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm
export BLOB_CLI_BIN := $(CARGO_TARGET_DIR)/debug/blob-storage
export BLOB_PACKAGE_VERIFIER := $(CARGO_TARGET_DIR)/debug/blob-package-verify
export BLOB_EXPECTED_HOST_RELEASE := $(shell perl scripts/release/release-data.pl package-version ic-blob-storage-canister)
export BLOB_BROWSER_NODE ?= node
export BLOB_BROWSER_NPM ?= npm
BLOB_SDK_INPUTS_BYTES ?= 10485760
VERSION ?=
RELEASE := bash scripts/release/release.sh
export RELEASE_DELIVERY ?= direct
export PUBLISH_PACKAGE
export RELEASE_GROUP
ifneq ($(filter version ensure-clean release-plan release-tag-check publish publish-dry-run $(SHARED_RELEASE_TARGETS),$(MAKECMDGOALS)),)
ifeq ($(filter contracts client engine,$(RELEASE_GROUP)),)
$(error Select RELEASE_GROUP=contracts, client or engine)
endif
ifneq ($(words $(RELEASE_GROUP)),1)
$(error Select exactly one RELEASE_GROUP)
endif
endif
release-patch release-minor release-major release-resume: export BLOB_RELEASE_PREPARE := 1
export RELEASE_KIND RELEASE_PREVIOUS RELEASE_VERSION RELEASE_DATE RELEASE_SOURCE RELEASE_COMMIT RELEASE_REMOTE RELEASE_BRANCH
SHELLCHECK ?= shellcheck
override CI_TARGETS = $(shell perl scripts/release/validation.pl stages)
CI_INPUTS := Cargo.toml Cargo.lock rust-toolchain.toml .shared-tooling.snapshot Makefile ci/tool-versions.env ci/ic-tools.tsv ci/release-delivery-policy.txt ci/cargo-release-groups.json ci/cargo-dependency-boundaries.json ci/candid-boundary.json ci/external-ic-dependencies.txt ci/cbor-dependencies.json \
	ci/release-validation.json ci/release-stage-reuse.json ci/release-package-scopes.json scripts/release/validation.pl scripts/release/stage.pl scripts/release/validate.sh scripts/ci/test-release-validation.sh \
	make/consumer.mk make/tools.mk make/release.mk make/rust-format.mk make/execution.mk scripts/ci/run-rust-formatting.pl scripts/ci/check-rust-compiler.sh \
	tests/browser/.nvmrc tests/browser/package.json tests/browser/package-lock.json \
	$(wildcard releases/*/CHANGELOG.md releases/*/release.json crates/*/Cargo.toml canisters/*/Cargo.toml canisters/tests/*/Cargo.toml tests/pocketic/Cargo.toml tests/protocol/Cargo.toml)

.PHONY: help version deps cloc shared-tooling-check dependency-pins-check check clippy docs-check test test-native test-pocketic test-browser test-browser-store test-browser-transport test-browser-standalone test-sdk-probe test-sdk-inputs test-fixture test-standalone build-standalone test-admission-resources test-read-resources test-funding-receipt-resources wasm-check \
	build package package-regression package-verifier clean shell-check release-check probe-check ci validate release-verify test-browser-publication test-browser-bootstrap test-browser-launcher test-browser-native \
	release-plan ensure-clean \
	release-version release-preflight release-prepare-version release-prepared-check release-files \
	release-commit-check release-committed-check release-tagged-check release-push-check \
	release-tag-check publish publish-dry-run install-hooks release-tools-check hooks-check evidence-check
.PHONY: test-native-host test-wasm-opt test-pocketic-client test-pocketic-engine test-pocketic-prepare compiler-check compiler-graphs-check tasks
.PHONY: release-metadata-check release-validation-check
.PHONY: contracts-boundary-check documentation-links-check release-commands-check shared-tooling-tests tooling-evidence-check
.PHONY: install-testkit testkit-check
.PHONY: install-release-editor release-editor-check

help:
	@echo "release-tools-check          Check ShellCheck, pinned cargo-sort and rustfmt without building"
	@echo "test-native-host             CLI/examples and local installation/restoration probe checks"
	@echo "compiler-check               Admit the declared Rust/Cargo pin and all member support floors offline"
	@echo "compiler-graphs-check        Check isolated public native/Wasm graphs on the declared compiler offline"
	@echo "tasks                        Show the shared maintenance task catalog"
	@echo "browser-tools-check          Offline exact browser Node/npm and manifest/lock checks"
	@echo "install-tools / tools-check   Explicit local tool installation / offline verification"
	@echo "install-release-editor / release-editor-check  Prepare or admit the locked IC Build editor"
	@echo "install-host-tools / host-tools-check   Pinned repo-local jq/yq/rg/cloc setup / verification"
	@echo "install-ic-tools / ic-tools-check       Pinned repo-local IC setup / verification"
	@echo "install-rust-tools / rust-tools-check   Pinned shared Cargo tools setup / verification"
	@echo "install-testkit / testkit-check         Locked Testkit CLI and owned PocketIC setup / offline check"
	@echo "deps                         Fetch locked Rust dependencies (network)"
	@echo "cloc                         Offline categorized Rust LOC for every workspace member"
	@echo "test-funding-receipt-resources  Measure populated receipt confirmation and restore (opt-in)"
	@echo "shared-tooling-check          Verify the reviewed shared snapshot offline"
	@echo "shared-tooling-tests          Exercise consumer tool, evidence, lockfile and metadata checks offline"
	@echo "documentation-links-check     Check local Markdown targets without a build or network"
	@echo "tooling-evidence-check        Check retained tooling archive selection and failure behavior"
	@echo "release-commands-check        Check Make routing with a substitute release runner"
	@echo "dependency-pins-check         Check dependency selectors and workspace lockfiles offline"
	@echo "fmt / fmt-check              Format Rust or check formatting"
	@echo "check / clippy / test         Workspace aggregates; release or explicit validation only"
	@echo "test-native / test-pocketic   Native core tests or local IC fixtures"
	@echo "build-standalone / test-standalone   Standalone host Wasm or focused local IC tests"
	@echo "test-browser                  Opt-in Chromium certificate/IndexedDB evidence"
	@echo "test-browser-store            Opt-in Chromium journal persistence without Rust builds"
	@echo "test-browser-publication      Opt-in offline frozen-file checks; BLOB_PUBLICATION_REPORT=NEW_DIRECTORY"
	@echo "test-browser-transport        Opt-in owned TLS transport checks; BLOB_BROWSER_TRANSPORT_REPORT=NEW_DIRECTORY"
	@echo "test-browser-bootstrap        Opt-in owned worker/IndexedDB checks; BLOB_BOOTSTRAP_REPORT=NEW_DIRECTORY"
	@echo "test-browser-launcher         Opt-in native Chromium/profile/body checks; BLOB_LAUNCHER_REPORT=NEW_DIRECTORY"
	@echo "test-browser-native           Owned native pipe checks; BLOB_NATIVE_REPORT=NEW_DIRECTORY"
	@echo "test-browser-standalone       Opt-in local standalone upload/verified download rehearsal"
	@echo "test-sdk-probe                Opt-in local SDK fault probe; BLOB_SDK_PROBE_REPORT=NEW_DIRECTORY"
	@echo "test-sdk-inputs               Opt-in offline native/browser handoff; BLOB_SDK_INPUTS_REPORT=NEW_DIRECTORY"
	@echo "test-admission-resources     Local admission bounds and Wasm resource report"
	@echo "test-read-resources          Local read-slot and Wasm resource report"
	@echo "clean                        Explicitly remove build artifacts"
	@echo "docs-check / wasm-check       Check docs or the Wasm library build"
	@echo "test-wasm-opt                Qualify retained original/O3/Os/Oz installation and restore"
	@echo "probe-check                  Verify retained Caffeine probe artifacts offline"
	@echo "evidence-check               Check retained evidence hashes without Rust builds"
	@echo "ci / validate                Fetch locked dependencies, then validate"
	@echo "release-check                Test release tooling without publication"
	@echo "release-metadata-check       Focused real group renderer/rollback checks without publication"
	@echo "Release/publication requires RELEASE_GROUP=contracts, client or engine"
	@echo "release-plan VERSION=minor   Preview the selected group without effects"
	@echo "release-{patch,minor,major}   Maintainer: freeze group, validate, commit, tag, push"
	@echo "release-resume VERSION=x.y.z Maintainer: reconcile a saved release"
	@echo "release-batch-{status,resume,reconcile,retire} BATCH=id   Shared frozen-batch operations"
	@echo "install-hooks                Enable the repository-local formatting hook"
	@echo "publish / publish-dry-run     Separately publish or verify registry upload"

version:
	@$(RELEASE) version

deps:
	+cargo fetch --locked

install-testkit:
	+bash scripts/dev/testkit-server.sh setup

testkit-check:
	+bash scripts/dev/testkit-server.sh check

install-release-editor:
	@test "$(BLOB_PREPARED_TOOLS_ROOT)" = "$(CURDIR)" || { echo 'Borrowed prepared tools are check-only.' >&2; exit 1; }
	+bash scripts/dev/install-rust-tools.sh $(RELEASE_EDITOR_ARGS)

release-editor-check:
	+@bash scripts/dev/install-rust-tools.sh $(RELEASE_EDITOR_ARGS) --check

# Shared owns generic formatter, digest, installer and release-runner matrices.
# Blob retains actual Make/hook/tool admission and archive/retention callers below.
shared-tooling-tests:
	+bash scripts/ci/test-consumer-fixture-completion.sh
	+bash scripts/ci/test-rust-compiler.sh
	+bash scripts/ci/check-validation-logging.sh
	+$(MAKE) --no-print-directory tooling-evidence-check
	+bash scripts/ci/test-testkit-commands.sh
	+$(MAKE) --no-print-directory release-validation-check

tooling-evidence-check:
	bash scripts/ci/test-tooling-evidence.sh

documentation-links-check:
	@set -o pipefail; find docs audits rules releases -type f -name '*.md' -print0 | \
		xargs -0 perl scripts/ci/check-documentation-links.pl --root . *.md

release-commands-check:
	@# The inner captured hooks use the standard runner; outer dispatch is checked below.
	@for group in contracts client engine; do \
		RELEASE_BATCH_ACTIVE=1 RELEASE_GROUP="$$group" bash scripts/ci/check-release-commands.sh "$(CURDIR)" Cargo.toml scripts/release/release-data.pl ci/release-delivery-policy.txt make/consumer.mk make/tools.mk make/release.mk make/rust-format.mk make/execution.mk scripts/ci/check-make-execution.sh scripts/ci/check-release-delivery.sh scripts/ci/verify-file-checksum.sh scripts/ci/run-formatting.sh scripts/ci/run-rust-formatting.pl || exit $$?; done
	bash scripts/release/test-release.sh --frozen-routing

dependency-pins-check:
	+@set -e; node_version="$$(cat tests/browser/.nvmrc)"; \
		npm_version="$$(jq -er '.packageManager | capture("^npm@(?<version>[0-9]+\\.[0-9]+\\.[0-9]+)$$").version' tests/browser/package.json)"; \
		bash scripts/ci/check-dependency-pins.sh --cargo-inheritance \
			--npm-root tests/browser --node-version "$$node_version" --npm-version "$$npm_version"

release-tools-check: format-tools-check release-editor-check
	@command -v "$(SHELLCHECK)" >/dev/null 2>&1 && "$(SHELLCHECK)" --version >/dev/null 2>&1 || { echo "Prepare ShellCheck: sudo apt-get install shellcheck (Debian/Ubuntu) or brew install shellcheck (macOS)." >&2; echo "Select an existing executable with SHELLCHECK=/absolute/path/to/shellcheck." >&2; exit 1; }

hooks-check:
	+bash scripts/ci/check-format-hooks.sh

.PHONY: platform-boundary-check
platform-boundary-check:
	+perl scripts/ci/check-candid-boundary.pl --cbor "$(CURDIR)" ci/candid-boundary.json crates canisters tests
	+bash scripts/ci/check-cargo-dependency-boundaries.sh --consumer "$(CURDIR)" --external-ic

contracts-boundary-check:
	+bash scripts/ci/check-runtime-free-contracts.sh
	+bash scripts/ci/test-runtime-free-contracts.sh

# Optional native compilation command; the automatic gate uses matching Clippy.
check-run:
	+@packages="$$(perl scripts/release/validation.pl packages check)" || exit $$?; \
		cargo check --offline --locked $$packages --all-targets --all-features

clippy-run:
	+@packages="$$(perl scripts/release/validation.pl packages clippy)" || exit $$?; \
		cargo clippy --offline --locked $$packages --all-targets --all-features -- -D warnings

probe-check-run:
	+cargo build --offline --locked -p ic-blob-storage-cli --bin caffeine-probe
	@for run in docs/evidence/caffeine-probes/runs/*; do \
		"$(CARGO_TARGET_DIR)/debug/caffeine-probe" verify "$$run" || exit $$?; \
	done
	+$(MAKE) --no-print-directory evidence-check

# Check Blob evidence itself; Shared owns the verifier implementation matrix.
evidence-check:
	bash scripts/ci/verify-evidence-checksums.sh docs/evidence/caffeine-probes/local/SHA256SUMS docs/evidence/caffeine-probes/deployed/SHA256SUMS

docs-check-run:
	+@packages="$$(perl scripts/release/validation.pl packages docs-check)" || exit $$?; \
		RUSTDOCFLAGS="-D warnings" cargo doc --offline --locked $$packages --all-features --no-deps

test:
	+$(MAKE) --no-print-directory test-native
	+$(MAKE) --no-print-directory test-pocketic

test-native-run:
	+@packages="$$(perl scripts/release/validation.pl packages test-native)" || exit $$?; \
		bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked $$packages --all-features

compiler-check:
	+bash scripts/ci/check-rust-compiler.sh

# The tooling job's all-target/all-feature Clippy covers the full native roster.
# These checks keep default features and supported Wasm libraries independent.
compiler-graphs-check: compiler-check
	+cargo check --offline --locked -p ic-blob-storage-contracts --lib
	+cargo check --offline --locked -p ic-blob-storage --lib
	+cargo check --offline --locked -p ic-blob-storage-client --lib
	+cargo check --offline --locked --target wasm32-unknown-unknown -p ic-blob-storage-contracts --lib
	+cargo check --offline --locked --target wasm32-unknown-unknown --workspace --exclude ic-blob-storage-cli --exclude ic-blob-storage-pocketic-tests --exclude blob-test-protocol --lib --all-features

tasks:
	@cat tasks/README.md

test-native-host: contracts-boundary-check
	+$(MAKE) --no-print-directory tools-check
	+$(MAKE) --no-print-directory build-standalone
	+cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-storage-probe -p blob-consumer-probe --lib
	+cargo build --offline --locked -p ic-blob-storage-cli
	+bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-cli --bins
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone standalone_installation_cli:: -- --test-threads=1
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test storage storage_resources::reads::restoration_reads_are_attributed_only_to_the_operator_reopen_window -- --exact --test-threads=1
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test storage storage_tenants::client:: -- --test-threads=1

test-fixture:
	+cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-authority-probe -p blob-admission-probe -p blob-storage-probe -p blob-consumer-probe -p blob-gateway-source -p blob-funding-probe --lib

test-pocketic-prepare:
	+$(MAKE) --no-print-directory testkit-check
	+$(MAKE) --no-print-directory test-fixture
	+$(MAKE) --no-print-directory build-standalone
	+cargo build --offline --locked -p ic-blob-storage-cli

# These disjoint selections exhaust Cargo's test roster (checked by the plan
# fixture). Client keeps its installed host, CLI and replicated-client boundary;
# the remaining Engine scenarios never run twice in a complete gate.
test-pocketic-run:
	+@partitions="$$(perl scripts/release/validation.pl pocketic-targets)" || exit $$?; \
		$(MAKE) --no-print-directory -j1 -S $$partitions

# One Make graph visits the common preparation once, including two-goal calls.
test-pocketic-client: test-pocketic-client-run
test-pocketic-engine: test-pocketic-engine-run

test-pocketic-client-run: test-pocketic-prepare
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test storage storage_tenants::client::

test-pocketic-engine-run: test-pocketic-prepare
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --lib --test admission --test authority --test content --test funding --test gateway_sync --test journey --test obligations --test operator --test storage --test uploads -- --skip storage_tenants::client::
	+cargo test --offline --locked -p ic-blob-storage-pocketic-tests --doc

build-standalone:
	+cargo build --offline --locked --release --target wasm32-unknown-unknown -p ic-blob-storage-canister --lib

test-wasm-opt:
	+$(MAKE) --no-print-directory tools-check
	+$(MAKE) --no-print-directory build-standalone
	+cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-storage-probe --lib
	+cargo build --offline --locked -p ic-blob-storage-cli
	+bash scripts/ci/qualify-blob-wasm-opt.sh

test-standalone:
	+$(MAKE) --no-print-directory testkit-check
	+cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-gateway-source -p blob-storage-probe -p blob-consumer-probe --lib
	+$(MAKE) --no-print-directory build-standalone
	+cargo build --offline --locked -p ic-blob-storage-cli
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone -- --test-threads=1

# Browser tooling is explicitly provisioned; this target performs no downloads.
.PHONY: browser-tools-check
browser-tools-check:
	bash scripts/ci/check-browser-tools.sh

test-sdk-probe test-sdk-inputs test-browser-transport test-browser-publication \
test-browser-store test-browser-bootstrap test-browser-launcher test-browser-native \
test-browser-standalone test-browser: browser-tools-check

test-sdk-probe:
	@test -n "$(BLOB_SDK_PROBE_REPORT)" || { echo 'Set BLOB_SDK_PROBE_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	+cargo build --offline --locked -p ic-blob-storage-contracts --example verify_download
	$(BLOB_BROWSER_NODE) target/browser/sdk-probe.mjs "$(BLOB_SDK_PROBE_REPORT)" "$(CARGO_TARGET_DIR)/debug/examples/verify_download"

test-sdk-inputs:
	@test -n "$(BLOB_SDK_INPUTS_REPORT)" || { echo 'Set BLOB_SDK_INPUTS_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	+cargo build --offline --locked -p ic-blob-storage-cli --bin blob-storage
	$(BLOB_BROWSER_NODE) target/browser/native-inputs.mjs "$(BLOB_SDK_INPUTS_REPORT)" "$(CARGO_TARGET_DIR)/debug/blob-storage" "$(BLOB_SDK_INPUTS_BYTES)"

test-browser-transport:
	@test -n "$(BLOB_BROWSER_TRANSPORT_REPORT)" || { echo 'Set BLOB_BROWSER_TRANSPORT_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) tests/browser/transport.mjs "$(BLOB_BROWSER_TRANSPORT_REPORT)"

test-browser-publication:
	@test -n "$(BLOB_PUBLICATION_REPORT)" || { echo 'Set BLOB_PUBLICATION_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) target/browser/publication.mjs "$(BLOB_PUBLICATION_REPORT)"

test-browser-store:
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) tests/browser/store.mjs

test-browser-bootstrap:
	@test -n "$(BLOB_BOOTSTRAP_REPORT)" || { echo 'Set BLOB_BOOTSTRAP_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) tests/browser/bootstrap.mjs "$(BLOB_BOOTSTRAP_REPORT)"

test-browser-launcher:
	@test -n "$(BLOB_LAUNCHER_REPORT)" || { echo 'Set BLOB_LAUNCHER_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) tests/browser/launcher.mjs "$(BLOB_LAUNCHER_REPORT)"

test-browser-native:
	@test -n "$(BLOB_NATIVE_REPORT)" || { echo 'Set BLOB_NATIVE_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/native.mjs "$(BLOB_NATIVE_REPORT)"

test-browser-standalone:
	$(BLOB_BROWSER_NODE) --test tests/browser/control.test.mjs
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	+cargo build --offline --locked --release --target wasm32-unknown-unknown -p ic-blob-storage-canister --lib
	+cargo build --offline --locked -p ic-blob-storage-cli --bin blob-storage
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone standalone_browser:: -- --ignored --test-threads=1

test-browser:
	$(BLOB_BROWSER_NODE) --test tests/browser/response.test.mjs
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) tests/browser/store.mjs
	+cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-storage-probe -p blob-consumer-probe --lib
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test storage chromium_certificate_intent -- --ignored --test-threads=1

test-admission-resources:
	+cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-admission-probe --lib
	@mkdir -p target/reports
	+BLOB_ADMISSION_RESOURCE_REPORT="$(CURDIR)/target/reports/admission-resources.json" \
	BLOB_ADMISSION_HISTORY_REPORT="$(CURDIR)/target/reports/admission-history.json" \
	BLOB_RELEASE_HISTORY_REPORT="$(CURDIR)/target/reports/release-history.json" \
	BLOB_REFERENCE_HISTORY_REPORT="$(CURDIR)/target/reports/reference-history.json" \
	BLOB_DESCRIPTOR_RESOURCE_REPORT="$(CURDIR)/target/reports/descriptor-resources.json" \
	bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test admission admission_resources -- --test-threads=2
	@echo "Local resource reports in target/reports/: admission-resources.json, admission-history.json, release-history.json, reference-history.json, descriptor-resources.json (not provider pricing)"

test-funding-receipt-resources:
	@test -n "$(BLOB_FUNDING_RECEIPT_PROFILE)" || { echo "Set BLOB_FUNDING_RECEIPT_PROFILE to a fresh report directory." >&2; exit 1; }
	+cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-storage-probe --lib
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test storage storage_resources::credits::receipt_populated_confirmation_and_restoration_profile -- --ignored --exact --test-threads=1

test-read-resources:
	+cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-authority-probe -p blob-gateway-source --lib
	@mkdir -p target/reports
	+BLOB_READ_RESOURCE_REPORT="$(CURDIR)/target/reports/read-resources.json" \
	bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test journey readback -- --test-threads=2
	@echo "Local read report: target/reports/read-resources.json (not a production read protocol or provider pricing)"

wasm-check-run:
	+@packages="$$(perl scripts/release/validation.pl packages wasm-check)" || exit $$?; \
		cargo check --offline --locked $$packages --all-features --target wasm32-unknown-unknown

build:
	+cargo build --offline --locked -p ic-blob-storage --all-features

package-verifier:
	+cargo build --offline --locked -p ic-blob-storage-cli --features package-verification --bin blob-package-verify

package-regression-run: package-verifier
	+bash scripts/ci/test-library-packages.sh

package: package-verifier
	@test -f crates/ic-blob-storage-client/LICENSE && test ! -L crates/ic-blob-storage-client/LICENSE
	cmp LICENSE crates/ic-blob-storage-client/LICENSE
	+bash scripts/ci/verify-library-packages.sh

clean:
	+cargo clean

shell-check:
	@for script in scripts/release/*.sh scripts/release/tests/*.sh scripts/dev/*.sh scripts/ci/*.sh .githooks/pre-commit; do bash -n "$$script" || exit $$?; done
	"$(SHELLCHECK)" scripts/release/*.sh scripts/release/tests/*.sh scripts/dev/*.sh scripts/ci/*.sh .githooks/pre-commit
	@for script in scripts/release/*.pl scripts/ci/check-documentation-links.pl scripts/ci/prepare-cargo-stage-source.pl scripts/ci/run-validation-stage.pl scripts/ci/observe-cargo-stage.pl scripts/ci/seal-validation-stage.pl scripts/ci/check-cargo-stage-closure.pl; do perl -c "$$script" || exit $$?; done

release-check-run: release-commands-check
	bash scripts/release/test-release.sh

release-metadata-check: release-commands-check
	bash scripts/release/test-release.sh --metadata-only "$(if $(RELEASE_GROUP),$(RELEASE_GROUP),engine)"

release-validation-check:
	bash scripts/ci/test-release-validation.sh

ci:
	+@targets="$(CI_TARGETS)"; \
		[[ -n "$$targets" ]] || { echo 'Cannot read the full CI stage roster' >&2; exit 1; }; \
		env -u BLOB_RELEASE_VALIDATION_PLAN bash scripts/ci/run-validation-targets.sh --fail-fast \
		$(foreach input,$(CI_INPUTS),--input "$(input)") $$targets

validate: ci

release-verify:
	+VALIDATION_FAILURE_LOG_DIR="$$(git rev-parse --git-path release-state)/validation-failures" \
		bash scripts/release/validate.sh

release-plan:
	@$(RELEASE) plan "$(if $(VERSION),$(VERSION),patch)"

ensure-clean:
	@$(RELEASE) ensure-clean

release-version:
	@$(RELEASE) version

release-preflight:
	+@$(RELEASE) preflight

release-prepare-version:
	+@$(RELEASE) prepare

release-prepared-check:
	+@$(RELEASE) prepared-check

release-files:
	@$(RELEASE) files

release-commit-check:
	@$(RELEASE) commit-check

release-committed-check:
	@$(RELEASE) committed-check

release-tagged-check release-push-check:
	@$(RELEASE) tagged-check

release-tag-check:
	$(RELEASE) tag-check

publish:
	+$(RELEASE) publish

publish-dry-run:
	+$(RELEASE) publish --dry-run

# Opt-in host schema qualification; normal deployed builds never enable metadata.
.PHONY: test-wire-schema
test-wire-schema: browser-tools-check
	+cargo build --offline --locked -p ic-blob-storage-contracts --example wire_schema --features wire-schema
	mkdir -p target/wire-schema
	$(CARGO_TARGET_DIR)/debug/examples/wire_schema metadata > target/wire-schema/metadata.json
	$(CARGO_TARGET_DIR)/debug/examples/wire_schema client > target/wire-schema/client.ts
	$(CARGO_TARGET_DIR)/debug/examples/wire_schema adapter > target/wire-schema/blob-runtime.ts
	$(CARGO_TARGET_DIR)/debug/examples/wire_schema runtime > target/wire-schema/wire-runtime.ts
	$(BLOB_BROWSER_NODE) tests/browser/wire-schema.mjs

# Ordinary CI/commands execute their original recipes. Group releases may admit
# exact final-candidate stage evidence through the shared source/proof owners.
.PHONY: clippy-run release-check-run check-run probe-check-run docs-check-run test-native-run test-pocketic-run test-pocketic-client-run test-pocketic-engine-run wasm-check-run package-regression-run test-release-identity
clippy release-check check probe-check docs-check test-native test-pocketic wasm-check package-regression:
	+perl scripts/release/stage.pl "$@"

# Prospective suites exercise final versions. This small fresh check covers the
# actual intermediate release mixture without replaying either installed suite.
test-release-identity: testkit-check
	+$(MAKE) --no-print-directory build-standalone
	+cargo build --offline --locked -p ic-blob-storage-cli --bin blob-storage
	+bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage --lib ops::service::installation::tests::compiled_library_version_binds_installation_and_reopen -- --exact
	+bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-contracts --lib identity::caffeine::manifest::verification::ordered::checkpoint::tests::bounded_decoding_rejects_damage_and_inconsistent_counts_even_with_a_new_checksum -- --exact
	+bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone standalone_installation_cli::standalone_installs_cli_carrier_and_independently_rejects_wrong_actual_service -- --exact --test-threads=1
