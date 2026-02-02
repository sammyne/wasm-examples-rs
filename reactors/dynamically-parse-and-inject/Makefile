WORKDIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

OUT_DIR := $(WORKDIR)/_out

TARGET_DIR := $(WORKDIR)/../../target

.PHONY: all
all: $(OUT_DIR)/app-component.wasm

$(OUT_DIR):
	@mkdir -p $@

$(OUT_DIR)/app-component.wasm: $(OUT_DIR)/app.wasm
	@wasm-tools component new $< -o $@ --adapt /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.reactor.wasm

$(OUT_DIR)/app-component.wat: $(OUT_DIR)/app-component.wasm
	@wasm-tools print $< > $@

$(OUT_DIR)/app.wasm: app $(OUT_DIR)
	@cd $< && cargo build --release
	@cp $(TARGET_DIR)/wasm32-wasi/release/`grep '^name = ' $</Cargo.toml | awk -F'"' '{print $$2}' | sed 's/-/_/g'`.wasm $@

fmt:
	@cd app && cargo fmt	
	@cd cli && cargo fmt	

run: $(OUT_DIR)/app-component.wasm
	@cd cli && cargo run $<

to-wat: $(OUT_DIR)/app-component.wat

.PHONY: clean
clean:
	@rm -rf $(OUT_DIR)
