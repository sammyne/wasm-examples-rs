# 模块组合

## 项目结构说明

文件 | 说明
-----|-----------
adder | 加法器 0.1.0 版
adder-v2 | 加法器 0.2.0 版，依赖 adder
cli | 调用 adder-v2 打包所得 WASM 组件的命令行工具

## 快速开始

```bash
make run
```

> 具体涉及的操作参见 [Makefile](./Makefile)。

## 参考文献
- [The WebAssembly Component Model/Components in Rust](https://component-model.bytecodealliance.org/language-support/rust.html)
