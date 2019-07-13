all: force
	cargo build --all --all-features
	cargo test --all --all-features --no-run

check: force
	cargo check --all --all-features

test: force
	cargo test --all --all-features -- --nocapture

force:

