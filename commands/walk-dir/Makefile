OUT_DIR := ../../target/wasm32-wasip2/release

NAME := $(shell grep '^name' Cargo.toml | cut -d '"' -f 2)

all: $(OUT_DIR)/$(NAME).wasm

$(OUT_DIR)/$(NAME).wasm: src/main.rs
	cargo build -r --target wasm32-wasip2

# 将当前目录挂载到 WASM 的当前工作目录，以便 WASM 可以访问当前目录下的文件
run: $(OUT_DIR)/$(NAME).wasm
	wasmtime run --dir $(PWD)::. $<

.PHONY: clean
clean:
	rm -rf $(OUT_DIR)/$(NAME).wasm