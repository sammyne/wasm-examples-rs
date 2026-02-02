WORKDIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

OUT_DIR := $(WORKDIR)/_out

TARGET_DIR := $(WORKDIR)/../../target

.PHONY: all
all: $(OUT_DIR)/adder-component.wasm $(OUT_DIR)/calculator-component.wasm  $(OUT_DIR)/cli-component.wasm \
	$(OUT_DIR)/calculator-composed.wasm $(OUT_DIR)/app.wasm

$(OUT_DIR):
	mkdir -p $@

$(OUT_DIR)/%.wasm: % $(OUT_DIR)
	@cd $< && cargo build --release
	@cp $(TARGET_DIR)/wasm32-wasip1/release/`grep '^name = ' $</Cargo.toml | awk -F'"' '{print $$2}' | sed 's/-/_/g'`.wasm $@

$(OUT_DIR)/adder-component.wasm: $(OUT_DIR)/adder.wasm
	@wasm-tools component new $< -o $@ --adapt /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.reactor.wasm

$(OUT_DIR)/app.wasm: $(OUT_DIR)/cli-component.wasm $(OUT_DIR)/calculator-composed.wasm $(OUT_DIR)/adder-component.wasm 
	@wasm-tools compose $(word 1, $^) -d $(word 2, $^) -d $(word 3, $^) -o $@
	@wasm-tools validate $@ --features component-model

$(OUT_DIR)/calculator-component.wasm: $(OUT_DIR)/calculator.wasm
	@wasm-tools component new $< -o $@ --adapt /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.reactor.wasm

$(OUT_DIR)/calculator-composed.wasm: $(OUT_DIR)/calculator-component.wasm $(OUT_DIR)/adder-component.wasm
	@wasm-tools compose $(word 1, $^) -d $(word 2, $^) -o $@

$(OUT_DIR)/calculator.wasm: calculator/wit/deps/adder

$(OUT_DIR)/cli-component.wasm: $(OUT_DIR)/cli.wasm cli/wit/deps/adder cli/wit/deps/calculator
	@wasm-tools component new $< -o $@ --adapt /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.command.wasm

$(OUT_DIR)/cli.wasm: cli/wit/deps/adder cli/wit/deps/calculator

calculator/wit/deps/adder: adder/wit
	@rm -rf $@
	@mkdir -p `dirname $@`
	@cp -r $< $@

cli/wit/deps/%: %/wit
	@rm -rf $@
	@mkdir -p `dirname $@`
	@cp -r $< $@

fmt:
	@cd adder && cargo fmt	
	@cd calculator && cargo fmt	
	@cd cli && cargo fmt	

run: $(OUT_DIR)/app.wasm
	@wasmtime run $<

.PHONY: clean
clean:
	@rm -rf $(OUT_DIR)
	@rm -rf adder/wit/deps
	@rm -rf calculator/wit/deps
	@rm -rf cli/wit/deps
