test-integration:
	cargo build
	prove -v

test:
	cargo test

test-all: test test-integration

reinstall:
	cargo uninstall alias-auto-add
	cargo install
