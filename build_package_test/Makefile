all: build test run install package

build:
	cargo build -p project
	@echo "cargo build worked ============================================\n"

test:
	cargo test -p project
	@echo "cargo test worked ============================================\n"

run:
	cargo run -p project
	@echo "cargo run worked ============================================\n"

install:
	cargo install --path project --force
	@echo "cargo install worked ============================================\n"

package:
	cargo package -p project --allow-dirty
	@echo "cargo package worked ============================================\n"