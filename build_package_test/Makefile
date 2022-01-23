all: build test run install package

build:
	cargo build
	@echo "cargo build worked ============================================\n"

test:
	cargo test
	@echo "cargo test worked ============================================\n"

run:
	cargo run
	@echo "cargo run worked ============================================\n"

install:
	cargo install --path .
	@echo "cargo install worked ============================================\n"

package:
	cargo package --allow-dirty
	@echo "cargo package worked ============================================\n"