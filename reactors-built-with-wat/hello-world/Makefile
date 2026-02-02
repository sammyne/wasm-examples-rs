BUILD_DIR = build

APP = $(BUILD_DIR)/app.wasm
HOST = ../../target/release/wat-hello-world-host

.PHONY: all
all: $(APP) $(BUILD_DIR)/app.wit

$(APP): guest.wat $(BUILD_DIR)
	wasm-tools parse $< -o $(APP)

$(BUILD_DIR):
	mkdir -p $@

$(BUILD_DIR)/app.wit: guest.wat $(BUILD_DIR)
	wasm-tools component wit $< > $@

$(HOST): host $(BUILD_DIR)/app.wit
	cd $< && cargo build -r

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)
	cd host && cargo clean

run: $(HOST) $(APP)
	$(HOST) -p $(APP) -s "hello world"