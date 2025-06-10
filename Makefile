test:
	cargo test

build:
	cargo build

clean:
	cargo clean

# For now until it takes CLI arguments
run:
	cargo run

doc:
	cargo doc --open --no-deps
