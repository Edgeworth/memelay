debug:
	RUST_BACKTRACE=1 RUST_LOG=info cargo +nightly run

release:
	RUST_BACKTRACE=1 RUST_LOG=info cargo +nightly run --release

test:
	RUST_BACKTRACE=1 cargo +nightly test --workspace --all-features --all-targets  -- --nocapture

fix:
	cargo +nightly fix --workspace --all-features --all-targets --edition-idioms
	cargo +nightly clippy --workspace --all-targets --all-features --fix -Z unstable-options
	cargo +nightly fmt

check: test
	cargo +nightly outdated --workspace
	cargo +nightly audit
	cargo +nightly udeps --all-features --all-targets --workspace

update-deps:
	rustup update
	rustup toolchain install nightly
	rustup component add clippy
	cargo +nightly install cargo-audit cargo-outdated cargo-bloat cargo-tree cargo-udeps
	cargo +nightly update
	cargo +nightly build --workspace --all-features --all-targets
