rust_build:
	cargo build --release

rust_run:
	cargo run

rust_test:
	cargo test

rust_format:
	cargo fmt

rust_lint:
	cargo clippy -- -D warnings

all: rust_build rust_run rust_test rust_format rust_lint


# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi