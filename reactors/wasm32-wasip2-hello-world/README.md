# wasm32-wasip2 示例程序

此项目演示基于 rust 的原生 `wasm32-wasip2` 构建目标构建出 WASM 组件，不再依赖 wasm-tools 工具执行最后的组件化操作将 `wasm32-p1` 目标构建出来的模块包装为最后可用的 WASM 组件。

> 项目依赖 rust@1.82.0 添加的 rust `wasm32-wasip2` 构建目标，具体变更参见 [这里](https://github.com/rust-lang/rust/pull/126967/)。

## 快速开始

### 1. 编译
```bash
make
```

### 2. 运行

```bash
make run
```

## 温馨提示
- wit-bindgen 假设组件 wit 的文件结构如下（具体可参考工具底层依赖的 [Resolve::push_path] 函数说明）
    ```bash
    |-wit 
      |-描述组件包接口的 wit 文件
      |-deps
        |-依赖组件的 wit 包，例如 hello/world.wit；如果只有一个包，也可以将 world.wit 放在 deps 根目录下
    ```

## 参考文献
- https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip2.html
- [Compiled wasm32-wasip2 component from simple code requires excessive WASI interfaces](https://github.com/rust-lang/rust/issues/133235)
- https://component-model.bytecodealliance.org/language-support/rust.html
- https://github.com/WebAssembly/component-model/issues/330

[Resolve::push_path]: https://docs.rs/wit-parser/latest/wit_parser/struct.Resolve.html#method.push_path
