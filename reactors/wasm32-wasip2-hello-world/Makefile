WORKDIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

OUT_DIR := $(WORKDIR)/_out

TARGET_DIR := $(WORKDIR)/../../target

.PHONY: all
all: $(OUT_DIR)/app.wasm

$(OUT_DIR):
	@mkdir -p $@

$(OUT_DIR)/app.wat: $(OUT_DIR)/app.wasm
	@wasm-tools print $< > $@

$(OUT_DIR)/app.wasm: guest $(OUT_DIR)
	@cd $< && cargo build --release
	@cp $(TARGET_DIR)/wasm32-wasip2/release/`grep '^name = ' $</Cargo.toml | awk -F'"' '{print $$2}' | sed 's/-/_/g'`.wasm $@

fix:
	@cd guest && cargo fix	
	@cd host && cargo fix	

fmt:
	@cd guest && cargo fmt	
	@cd host && cargo fmt	

run: $(OUT_DIR)/app.wasm
	@cd host && cargo run -- -p $< --name sammyne

to-wat: $(OUT_DIR)/app.wat

.PHONY: clean
clean:
	@rm -rf $(OUT_DIR)
