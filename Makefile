test-integration:
	cargo build
	prove -v

test:
	cargo test

test-all: test test-integration

reinstall:
	cargo uninstall alias-auto-add
	cargo install

package:
	cargo build --release
	version=`cat Cargo.toml | grep version | awk '{ gsub("\"", ""); print $$3 }'`; \
		xz --compress --to-stdout ./target/release/alias-auto-add > alias-auto-add_$${version}_osx_amd64.xz
