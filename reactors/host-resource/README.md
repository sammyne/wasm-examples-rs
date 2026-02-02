# Guest Resource

此项目演示使用 WIT 在 guest 侧定义的资源的方式。

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
- https://component-model.bytecodealliance.org/language-support/rust.html
- https://github.com/WebAssembly/component-model/issues/330

[Resolve::push_path]: https://docs.rs/wit-parser/latest/wit_parser/struct.Resolve.html#method.push_path
