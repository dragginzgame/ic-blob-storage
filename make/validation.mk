# Canonical leaf policy: ci and selected releases use the same stage declaration.
.PHONY: check-common check-library check-cli check-host check-app check-qualification check-browser prepare-installed artifact-cli
.PHONY: prepare-test-server installed-standalone installed-capacity installed-endpoints installed-signed installed-application installed-enrollment installed-callbacks installed-management test-release-validation
.NOTPARALLEL: check-browser

test-release-validation:
	perl scripts/release/tests/validation.pl
check-common: dependency-pins-check tooling-binding-check fmt-check documentation-links-check

check-library:
	CARGO_BUILD_WARNINGS=deny cargo clippy --locked -p ic-blob-storage --all-targets --all-features -- -D warnings
	CARGO_BUILD_WARNINGS=deny cargo test --locked -p ic-blob-storage --lib
	$(MAKE) --no-print-directory example-local test-example-local
	RUSTDOCFLAGS='-D warnings' cargo doc --locked -p ic-blob-storage --all-features --no-deps

check-cli:
	CARGO_BUILD_WARNINGS=deny cargo clippy --locked -p ic-blob-storage-cli --all-targets --all-features -- -D warnings
	$(MAKE) --no-print-directory test-cli
	RUSTDOCFLAGS='-D warnings' cargo doc --locked -p ic-blob-storage-cli --all-features --no-deps

check-host:
	CARGO_BUILD_WARNINGS=deny cargo clippy --locked -p ic-blob-storage-canister --all-targets --all-features -- -D warnings
	RUSTDOCFLAGS='-D warnings' cargo doc --locked -p ic-blob-storage-canister --all-features --no-deps

check-app:
	CARGO_BUILD_WARNINGS=deny cargo clippy --locked -p ic-blob-storage-reference-app --all-targets --all-features -- -D warnings
	RUSTDOCFLAGS='-D warnings' cargo doc --locked -p ic-blob-storage-reference-app --all-features --no-deps

check-qualification: clippy-qualification docs-qualification-check

check-browser: install-browser-tools test-browser

prepare-installed:
	bash scripts/release/prepare-installed.sh
