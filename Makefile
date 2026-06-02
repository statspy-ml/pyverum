PYTHON := python3.13

.PHONY: check test build develop publish-pypi publish-cargo publish clean

check:
	cargo check

test:
	cargo test

build:
	maturin build --release --features extension-module -i $(PYTHON)

develop:
	maturin develop --features extension-module

publish-pypi: build
	uv publish target/wheels/*.whl

publish-cargo:
	cargo publish

publish: publish-cargo publish-pypi

clean:
	cargo clean
	rm -rf target/wheels
