# Host Resource with Method Added

此项目演示 Host 侧实现的 WIT Resource 添加新方法是兼容性升级，能否运行旧的 WASM。

## 快速开始

### 1. 编译
```bash
make
```

### 2. 运行新版的 WASM

```bash
make run v2
```

样例输出如下

```bash
request-id for owned(false) and rep=(0)
say-hello returns [Record([("message", String("hello from sammyne with request-id=123 get greeting=nice to meet sammyne"))])]
```

### 2. 运行旧版的 WASM

```bash
make run v1
```

样例输出如下

```bash
request-id for owned(false) and rep=(0)
say-hello returns [Record([("message", String("hello from sammyne with request-id=123"))])]
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
- https://component-model.bytecodealliance.org/language-support/rust.html
- https://github.com/WebAssembly/component-model/issues/330

[Resolve::push_path]: https://docs.rs/wit-parser/latest/wit_parser/struct.Resolve.html#method.push_path
