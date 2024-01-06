PKG := execs

build:
	cargo build --release

develop:
	cargo build

install:
	cargo install --path . -f

clean:
	cargo clean

uninstall:
	@echo TODO: make uninstall

check:
	@echo TODO: make check
