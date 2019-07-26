all: force
	cargo build --all --all-features
	cargo build -p bluefire_frontend --all-features --target=wasm32-unknown-unknown
	cargo test --all --all-features --no-run

check: force
	cargo check --all --all-features
	cargo check -p bluefire_frontend --all-features --target=wasm32-unknown-unknown

test: force
	cargo test --all --all-features -- --nocapture

force:

