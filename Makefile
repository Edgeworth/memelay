debug:
	RUST_BACKTRACE=1 RUST_LOG=info cargo run

release:
	RUST_BACKTRACE=1 RUST_LOG=info cargo run --release

test:
	RUST_BACKTRACE=1 cargo test --workspace --all-features --all-targets

fix:
	cargo fix --workspace --all-features --all-targets --edition-idioms
	cargo clippy --workspace --all-targets --all-features --fix -Z unstable-options
	cargo fmt --all

check: test
	cargo outdated --workspace
	cargo audit
	cargo udeps --all-features --all-targets --workspace

update-deps:
	rustup update
	rustup toolchain install nightly
	rustup component add clippy
	CFG_RELEASE_CHANNEL=nightly CFG_RELEASE=nightly cargo install --force --branch master \
	  --git https://github.com/rust-lang/rustfmt.git --features rustfmt,cargo-fmt
	cargo install cargo-audit cargo-outdated cargo-bloat cargo-tree cargo-udeps
	cargo update
	cargo build --workspace --all-features --all-targets
