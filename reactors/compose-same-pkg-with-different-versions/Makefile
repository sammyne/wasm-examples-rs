WORKDIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

OUT_DIR := $(WORKDIR)/_out

TARGET_DIR := $(WORKDIR)/../../target

.PHONY: all
all: $(OUT_DIR)/app.wasm

$(OUT_DIR):
	mkdir -p $@

# $(OUT_DIR)/%.wasm 作为中间文件，会被 make 自动删除。ref: https://stackoverflow.com/a/53353824/10878967。 
$(OUT_DIR)/%-component.wasm: $(OUT_DIR)/%.wasm
	@wasm-tools component new $< -o $@ --adapt /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.reactor.wasm

$(OUT_DIR)/%.wasm: % $(OUT_DIR)
	@cd $< && cargo build --release
	@cp $(TARGET_DIR)/wasm32-wasi/release/`grep '^name = ' $</Cargo.toml | awk -F'"' '{print $$2}' | sed 's/-/_/g'`.wasm $@

$(OUT_DIR)/adder-v2.wasm: $(OUT_DIR)/adder.wasm adder-v2/wit/deps/adder

$(OUT_DIR)/app.wasm: $(OUT_DIR)/adder-v2-component.wasm $(OUT_DIR)/adder-component.wasm
	@wasm-tools compose $(word 1, $^) -d $(word 2, $^) -o $@

adder-v2/wit/deps/adder: adder/wit
	@rm -rf $@
	@mkdir -p `dirname $@`
	@cp -r $< $@

fmt:
	@cd adder && cargo fmt	
	@cd adder-v2 && cargo fmt	
	@cd cli && cargo fmt	

run: $(OUT_DIR)/app.wasm
	@cd cli && cargo run $<

.PHONY: clean
clean:
	@rm -rf $(OUT_DIR)
