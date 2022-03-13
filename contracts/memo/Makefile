SECRETCLI = docker exec -it secretdev /usr/bin/secretcli

.PHONY: all
all: clippy test

.PHONY: check
check:
	cargo check

.PHONY: clippy
clippy:
	cargo clippy

.PHONY: test
test: unit-test unit-test-receiver integration-test

.PHONY: unit-test
unit-test:
	cargo test

.PHONY: unit-test-receiver
unit-test-receiver:
	$(MAKE) -C tests/example-receiver unit-test

.PHONY: integration-test
integration-test: compile-optimized
	tests/integration.sh

.PHONY: list-code
list-code:
	$(SECRETCLI) query compute list-code

.PHONY: compile _compile
compile: _compile contract.wasm.gz
_compile:
	cargo build --target wasm32-unknown-unknown --locked
	cp ./target/wasm32-unknown-unknown/debug/*.wasm ./contract.wasm

.PHONY: compile-optimized _compile-optimized
compile-optimized: _compile-optimized
_compile-optimized:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
	@# The following line is not necessary, may work only on linux (extra size optimization)
	wasm-opt -Os ./target/wasm32-unknown-unknown/release/*.wasm -o memo.wasm
	cp ./target/wasm32-unknown-unknown/release/*.wasm ./build/

.PHONY: compile-optimized-reproducible
compile-optimized-reproducible:
	docker run --rm -v "$$(pwd)":/contract \
		--mount type=volume,source="$$(basename "$$(pwd)")_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		enigmampc/secret-contract-optimizer:1.0.4

.PHONY: start-server
start-server: # CTRL+C to stop
	docker run -it --rm \
		-p 26657:26657 -p 26656:26656 -p 1337:1337 -p 5000:5000 \
		-v $$(pwd):/root/code \
		--name secretdev enigmampc/secret-network-sw-dev:latest

.PHONY: schema
schema:
	cargo run --example schema

.PHONY: clean
clean:
	cargo clean
	rm -f ./contract.wasm ./contract.wasm.gz
	$(MAKE) -C tests/example-receiver clean

#.PHONY: compile-w-debug-print _compile-w-debug-print
#compile-w-debug-print: _compile-w-debug-print
#_compile-w-debug-print:
#	RUSTFLAGS='-C link-arg=-s' cargo +nightly build --release --target wasm32-unknown-unknown --features debug-print --locked
#	cd contracts/secretswap_pair && RUSTFLAGS='-C link-arg=-s' cargo build --release --features debug-print --target wasm32-unknown-unknown --locked
#	cd contracts/dummy_swap_data_receiver && RUSTFLAGS='-C link-arg=-s' cargo build --release --features debug-print --target wasm32-unknown-unknown --locked
#	cp ./target/wasm32-unknown-unknown/release/*.wasm ./build/


# token: 1
# factory: 2
# pair: 3