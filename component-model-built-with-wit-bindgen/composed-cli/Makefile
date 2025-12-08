WORKDIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

TARGET_DIR := $(WORKDIR)/../../target/wasm32-wasip2/release

all: $(TARGET_DIR)/composed-cli.wasm

$(TARGET_DIR)/composed-cli.wasm: $(TARGET_DIR)/composed-cli-cli.wasm $(TARGET_DIR)/composed-cli-guest.wasm
	wac plug $(word 1, $^) --plug $(word 2, $^) -o $@
	@wasm-tools validate $@ --features component-model	

$(TARGET_DIR)/composed-cli-cli.wasm: cli
	cd $< && cargo build -r

$(TARGET_DIR)/composed_cli_guest.wasm: guest
	cd $< && cargo build -r

$(TARGET_DIR)/composed-cli-guest.wasm: $(TARGET_DIR)/composed_cli_guest.wasm
	cp $< $@

run: $(TARGET_DIR)/composed-cli.wasm
	@wasmtime run $<

.PHONY: clean
clean:
	cd $(TARGET_DIR) && rm -rf composed-cli-cli.wasm composed_cli_guest.wasm composed-cli.wasm
