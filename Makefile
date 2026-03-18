.PHONY: build release clean

build:
	cargo build --release

release:
	@bash scripts/release.sh

clean:
	cargo clean

setup-dev:
	@bash scripts/setup-hooks.sh
