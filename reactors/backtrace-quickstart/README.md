# Guest Resource

此项目演示如何捕捉 WASM 侧的函数调用栈。

## 快速开始

### 1. 编译
```bash
make
```

### 2. 运行

```bash
make run
```

样例输出片段如下

```bash
[0] FrameInfo { module: Module { name: Some("backtrace_quickstart_app.wasm"), .. }, func_index: 11, func_name: Some("_ZN24backtrace_quickstart_app5greet17hb0cbc1f4599c33c4E"), func_start: FilePos(1849), instr: Some(FilePos(1891)), symbols: [] }
[1] FrameInfo { module: Module { name: Some("backtrace_quickstart_app.wasm"), .. }, func_index: 15, func_name: Some("sammyne:helloworld/greeter@1.0.0#say-hello"), func_start: FilePos(2078), instr: Some(FilePos(2122)), symbols: [] }
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
