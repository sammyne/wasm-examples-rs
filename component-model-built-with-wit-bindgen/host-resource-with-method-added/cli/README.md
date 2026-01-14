# hello-world

## 快速开始

### 1. 编译
```bash
cargo component build --release --lib
```

### 2. 运行

```bash
# hello_world.wasm 是上一步的编译产物
cargo run -- ../../target/wasm32-wasi/release/hello_world.wasm
```

## 参考文献
- https://component-model.bytecodealliance.org/language-support/rust.html
