use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;
use wasmtime::component::{Component, Linker, Val};
use wasmtime::{Config, Engine, Store};

mod bindings {
    wasmtime::component::bindgen!(in "../build");
}

fn main() -> anyhow::Result<()> {
    let Cli { path, string } = Cli::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let linker = Linker::new(&engine);

    // Add the command world (aka WASI CLI) to the linker
    // wasmtime_wasi::add_to_linker_sync(&mut linker).context("link command world")?;

    let mut store = Store::new(&engine, ());

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    let instance = linker
        .instantiate(&mut store, &component)
        .context("instantiate")?;

    let f = instance
        .get_func(&mut store, "length")
        .ok_or_else(|| anyhow::anyhow!("miss func"))?;

    let params = [Val::String(string)];
    let mut results = [Val::Bool(false)];
    f.call(&mut store, &params, &mut results).context("call")?;
    // post-return 清理 say-hello 关联的状态。
    f.post_return(&mut store).with_context(|| "post return")?;
    println!("length returns {results:?}");

    Ok(())
}

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Parser)]
#[clap(name = "hello-world-host", version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    /// WASM 组件的路径
    #[clap(short, long)]
    path: PathBuf,
    /// 输入参数。
    #[clap(short, long)]
    string: String,
}
