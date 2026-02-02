WORKDIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

OUT_DIR := $(WORKDIR)/_out

TARGET_DIR := $(WORKDIR)/../../target

.PHONY: all
all: $(OUT_DIR)/app-v1.wasm $(OUT_DIR)/app-v2.wasm

$(OUT_DIR):
	@mkdir -p $@

$(OUT_DIR)/app-%.wat: $(OUT_DIR)/app-%.wasm
	@wasm-tools print $< > $@

$(OUT_DIR)/app-%.wasm: app-% $(OUT_DIR)
	@cd $< && cargo build --release
	@cp $(TARGET_DIR)/wasm32-wasip2/release/`grep '^name = ' $</Cargo.toml | awk -F'"' '{print $$2}' | sed 's/-/_/g'`.wasm $@

fix:
	@cd app && cargo fix	
	@cd cli && cargo fix	

fmt:
	@cd app && cargo fmt	
	@cd cli && cargo fmt	

run-%: $(OUT_DIR)/app-%.wasm
	@cd cli && cargo run -- -p $< --name sammyne

to-wat: $(OUT_DIR)/app-component.wat

.PHONY: clean
clean:
	@rm -rf $(OUT_DIR)
