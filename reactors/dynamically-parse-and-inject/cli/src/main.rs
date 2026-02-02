use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use wasmtime::component::types::ComponentItem;
use wasmtime::component::{Component, Linker, LinkerInstance, Type, Val};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview1::WasiP1Ctx;
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

    parse_and_inject(&engine, &mut linker, &component).context("build and inject")?;

    let instance = linker
        .instantiate(&mut store, &component)
        .context("Failed to instantiate the example world")?;

    let f = instance
        .get_func(&mut store, "hello-world")
        .context("get func 'hello-world'")?;

    let mut returns = vec![Val::Bool(false)];
    f.call(&mut store, &[], &mut returns).context("call")?;

    let msg = match returns.pop().expect("miss returned value") {
        Val::String(s) => s,
        v => return Err(anyhow::anyhow!("unexpected return types: {v:?}")),
    };
    println!("{msg}");

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

/// 往 i 添加元素，例如函数，内嵌的组件实例等。
fn build_instance(
    engine: &Engine,
    i: &mut LinkerInstance<'_, WasiP1Ctx>,
    name: &str,
    item: &ComponentItem,
) -> anyhow::Result<()> {
    match item {
        ComponentItem::ComponentFunc(v) => {
            let expected_result_types: Vec<_> = v.results().collect();
            let f = name.to_owned();
            i.func_new(name, move |_store, params, results| {
                assert_eq!(
                    results.len(),
                    expected_result_types.len(),
                    "unexpected result buffer length"
                );

                for (t, r) in expected_result_types.iter().zip(results.iter_mut()) {
                    *r = match t {
                        Type::Bool => Val::Bool(false),
                        Type::S8 => Val::S8(0),
                        Type::U8 => Val::U8(1),
                        Type::S16 => Val::S16(2),
                        Type::U16 => Val::U16(3),
                        Type::S32 => Val::S32(4),
                        Type::U32 => Val::U32(5),
                        Type::S64 => Val::S64(6),
                        Type::U64 => Val::U64(7),
                        Type::Char => Val::Char(' '),
                        Type::String => Val::String("hello-world".to_owned()),
                        _ => todo!(),
                    };
                }

                let params_str = match format!("{params:?}")
                    .strip_prefix('[')
                    .map(|s| s.strip_suffix(']'))
                {
                    Some(Some(v)) => v.to_owned(),
                    _ => panic!("strip surrounding '[]' in params str"),
                };

                let results_str = match format!("{results:?}")
                    .strip_prefix('[')
                    .map(|s| s.strip_suffix(']'))
                {
                    Some(Some(v)) => v.to_owned(),
                    _ => panic!("strip surrounding '[]' in results str"),
                };

                println!("{f}({params_str}) -> ({results_str})");

                Ok(())
            })
            .with_context(|| format!("new func '{name}'"))
        }
        ComponentItem::CoreFunc(_) => todo!(),
        ComponentItem::Module(_) => todo!(),
        ComponentItem::Component(_) => todo!(),
        ComponentItem::ComponentInstance(v) => {
            let mut i = i
                .instance(name)
                .with_context(|| format!("new instance '{name}'"))?;
            for (name, item) in v.exports(engine) {
                build_instance(engine, &mut i, name, &item)
                    .with_context(|| format!("add item '{name}'"))?;
            }
            Ok(())
        }
        ComponentItem::Type(_) => todo!(),
        ComponentItem::Resource(_) => todo!(),
    }
}

/// 从 c 的 imports 解析出依赖项，并利用 linker 动态构造出这些依赖项。
fn parse_and_inject(
    engine: &Engine,
    linker: &mut Linker<WasiP1Ctx>,
    c: &Component,
) -> anyhow::Result<()> {
    const EXCLUDED_NAMESPACES: &[&str] = &["wasi"];

    let cc = c.component_type();

    let mut root = linker.root();
    for (name, item) in cc.imports(&engine) {
        if name
            .split_once(':')
            .map(|(lhs, _)| EXCLUDED_NAMESPACES.contains(&lhs))
            .unwrap_or_default()
        {
            continue;
        }
        build_instance(engine, &mut root, name, &item)
            .with_context(|| format!("build instance '{name}'"))?
    }

    Ok(())
}
