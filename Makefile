DESTDIR =
PREFIX = ${HOME}/.local

all: target/release/pct
build: target/release/pct

target/release/pct:
	cargo build --release

install: install-pct

install-pct: target/release/pct
	install -m755 -- target/release/pair-commit-tool "$(DESTDIR)$(PREFIX)/bin/"

test: target/release/pct
	cargo test --release

check: test

uninstall:
	-rm -f -- "$(DESTDIR)$(PREFIX)/bin/pair-commit-tool"

clean:
	cargo clean

.PHONY: all build target/release/pct install-pct \
	clean uninstall
