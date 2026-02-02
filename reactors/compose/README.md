# 模块组合

## 1. 构建
### 1.1. 构建 adder 组件
```bash
cd adder
cargo component build --release
cd -
```

### 1.2. 构建 calculator 组件
```bash
cd calculator
cargo component build --release
cd -
```

### 1.3. 构建 cli 应用
```bash
cd cli
cargo component build --release
cd -
```

## 2. 组合

```bash
cd ../../target/wasm32-wasi/release

# 组合 adder 和 calculator 模块
wasm-tools compose calculator.wasm -d adder.wasm -o calculator-composed.wasm

# 由于 wasm-tools 尚不支持间接依赖，所以下述命令不行
#wasm-tools compose cli.wasm -d adder.wasm -d calculator.wasm -o cli-composed.wasm
# 而以下命令可以
wasm-tools compose cli.wasm -d calculator-composed.wasm -o cli-composed.wasm
```

> 温馨提示：wasm-tools 尚不支持间接依赖

## 3. 运行

```bash
wasmtime run cli-composed.wasm
```

## 参考文献
- [The WebAssembly Component Model/Components in Rust](https://component-model.bytecodealliance.org/language-support/rust.html)
