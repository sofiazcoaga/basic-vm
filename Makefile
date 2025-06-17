test:
	cargo test

build:
	cargo build

clean:
	cargo clean

run:
	cargo run -- $(path)

doc:
	cargo doc --open --no-deps
