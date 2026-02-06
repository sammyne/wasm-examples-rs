use wasmtime::{Engine, Result, Store, Config};
use wasmtime::component::{ResourceTable, Linker, Component};
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi::p2::bindings::Command;

// This example is an example shim of executing a component based on the
// command line arguments provided to this program.
#[tokio::main]
async fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    // Configure and create `Engine`
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;

    // Configure a `Linker` with WASI, compile a component based on
    // command line arguments, and then pre-instantiate it.
    let mut linker = Linker::<MyState>::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
    let component = Component::from_file(&engine, &args[0])?;


    // Configure a `WasiCtx` based on this program's environment. Then
    // build a `Store` to instantiate into.
    let mut builder = WasiCtx::builder();
    builder.inherit_stdio().inherit_env().args(&args);
    let mut store = Store::new(
        &engine,
        MyState {
            ctx: builder.build(),
            table: ResourceTable::new(),
        },
    );

    // Instantiate the component and we're off to the races.
    let command = Command::instantiate_async(&mut store, &component, &linker).await?;
    let program_result = command.wasi_cli_run().call_run(&mut store).await?;
    match program_result {
        Ok(()) => Ok(()),
        Err(()) => std::process::exit(1),
    }
}

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.ctx, table: &mut self.table }
    }
}