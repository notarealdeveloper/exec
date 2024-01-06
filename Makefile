PKG := exec
TARGET :=
# use this if you want to build against musl libc
# TARGET := --target x86_64-unknown-linux-musl

build:
	cargo build $(TARGET) --release

develop:
	cargo build $(TARGET)

install:
	cargo install $(TARGET) --path . -f

clean:
	cargo clean

uninstall:
	cargo uninstall

check:
	if [[ $$(printf "aaa\nbbb\nccc\n" | after  'b+') = ccc ]]; then echo yes; else echo no; fi
	if [[ $$(printf "aaa\nbbb\nccc\n" | before 'b+') = aaa ]]; then echo yes; else echo no; fi
