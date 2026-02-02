use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::WasiCtxBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let App { path } = App::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    // Add the command world (aka WASI CLI) to the linker
    wasmtime_wasi::add_to_linker_async(&mut linker).context("Failed to link command world")?;
    let ctx = WasiCtxBuilder::new().inherit_stdout().build_p1();
    let mut store = Store::new(&engine, ctx);

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    let instance = linker
        .instantiate_async(&mut store, &component)
        .await
        .context("instantiate")?;

    const INTERFACE: &str = "docs:adder/add@0.2.0";
    const FUNC: &str = "add";

    let f = instance
        .exports(&mut store)
        .instance(INTERFACE)
        .with_context(|| format!("interface '{INTERFACE}' not found"))?
        .func(FUNC)
        .with_context(|| format!("func '{FUNC} not found'"))?;

    let params = [Val::U32(1), Val::U32(2)];
    let mut results = [Val::Bool(false)];
    f.call_async(&mut store, &params, &mut results)
        .await
        .context("call func")?;

    match &results[0] {
        Val::U32(v) => println!("sum = {}", v),
        v => anyhow::bail!("unexpected return type: {v:?}"),
    }

    Ok(())
}

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Parser)]
#[clap(name = "hello-world-host", version = env!("CARGO_PKG_VERSION"))]
struct App {
    /// The path to the component.
    #[clap(value_name = "COMPONENT_PATH")]
    path: PathBuf,
}
