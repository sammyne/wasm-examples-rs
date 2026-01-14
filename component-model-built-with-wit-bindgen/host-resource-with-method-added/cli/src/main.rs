use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;
use wasmtime::component::{Component, HasSelf, Linker, Resource, ResourceAny, Val};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

mod bindings {
    wasmtime::component::bindgen!(in "../app-v2/wit");
}

use crate::bindings::sammyne::helloworld::types::Host;
use crate::bindings::sammyne::helloworld::types::{Context, HostContext};

fn main() -> anyhow::Result<()> {
    let Cli { path, name } = Cli::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    // Add the command world (aka WASI CLI) to the linker
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker).context("link command world")?;
    bindings::Helloworld::add_to_linker::<_, HasSelf<_>>(&mut linker, |v: &mut MyHost| v)
        .context("helloworld add-to-linker")?;

    let mut store = Store::new(&engine, MyHost::new());

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    const INTERFACE: &str = "sammyne:helloworld/greeter@1.0.0";
    const FUNC_NAME: &str = "say-hello";

    let instance = linker
        .instantiate(&mut store, &component)
        .context("instantiate")?;

    let owned_ctx: Resource<Context> = Resource::new_own(0);
    let ctx = ResourceAny::try_from_resource(owned_ctx, &mut store).context("borrow ctx")?;

    // let f = instance
    //     .exports(&mut store)
    //     .instance(INTERFACE)
    //     .ok_or_else(|| anyhow::anyhow!("miss interface: {INTERFACE}"))?
    //     .func(FUNC_NAME)
    //     .with_context(|| format!("miss func '{FUNC_NAME}'"))?;

    let f = {
        let ii = instance
            .get_export_index(&mut store, None, INTERFACE)
            .ok_or_else(|| anyhow::anyhow!("miss interface: {INTERFACE}"))?;

        let fi = instance
            .get_export_index(&mut store, Some(&ii), FUNC_NAME)
            .with_context(|| format!("miss export-index for func '{FUNC_NAME}'"))?;

        instance
            .get_func(&mut store, fi)
            .with_context(|| format!("miss func '{FUNC_NAME}'"))?
    };

    let params = [Val::Resource(ctx), new_hello_request(name.clone())];
    let mut results = [Val::Bool(false)];
    f.call(&mut store, &params, &mut results).context("call")?;
    // post-return 清理 say-hello 关联的状态。
    f.post_return(&mut store)
        .with_context(|| format!("post return '{FUNC_NAME}'"))?;
    println!("say-hello returns {results:?}");

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
    /// greet 函数的入参依赖的 name 字段。
    #[clap(short, long)]
    name: String,
}

fn new_hello_request(name: String) -> Val {
    Val::Record(vec![("name".to_owned(), Val::String(name))])
}

struct MyHost {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl MyHost {
    fn new() -> Self {
        Self {
            ctx: WasiCtxBuilder::new().inherit_stdout().build(),
            table: ResourceTable::new(),
        }
    }
}

impl Host for MyHost {}

impl HostContext for MyHost {
    fn request_id(&mut self, self_: wasmtime::component::Resource<Context>) -> i64 {
        println!(
            "request-id for owned({}) and rep=({})",
            self_.owned(),
            self_.rep()
        );
        123
    }

    fn greet(&mut self, _self_: wasmtime::component::Resource<Context>, name: String) -> String {
        format!("nice to meet {name}")
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Context>) -> wasmtime::Result<()> {
        println!("drop rep={}", rep.rep());
        Ok(())
    }
}

impl WasiView for MyHost {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}
