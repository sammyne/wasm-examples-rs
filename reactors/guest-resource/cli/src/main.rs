use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use wasmtime::component::{Component, Linker, Val};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::WasiCtxBuilder;

fn main() -> anyhow::Result<()> {
    let App { path } = App::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    // Add the command world (aka WASI CLI) to the linker
    wasmtime_wasi::add_to_linker_sync(&mut linker).context("link command world")?;

    let ctx = WasiCtxBuilder::new().inherit_stdout().build_p1();
    let mut store = Store::new(&engine, ctx);

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    let instance = linker
        .instantiate(&mut store, &component)
        .context("Failed to instantiate the example world")?;

    let f = instance
        .exports(&mut store)
        .instance("helloworld:example/api")
        .context("cannot find exported interface 'api'")?
        .func("hi")
        .context("cannot find func 'hi' in interface 'api'")?;

    let params = [Val::String("sammyne".to_owned())];
    f.call(&mut store, &params, &mut []).context("call")?;

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
