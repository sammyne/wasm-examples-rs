
OUT_DIR := ../../target/wasm32-wasip2/release

all: $(OUT_DIR)/command-hello-world.wasm

$(OUT_DIR)/command-hello-world.wasm: src/main.rs
	cargo build -r

run: $(OUT_DIR)/command-hello-world.wasm
	wasmtime run $<
