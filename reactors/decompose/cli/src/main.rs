use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use wasmtime::component::types::ComponentItem;
use wasmtime::component::Component;
use wasmtime::{Config, Engine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let App { path } = App::parse();

    let mut config = Config::default();
    config.wasm_component_model(true).async_support(true);
    let engine = Engine::new(&config)?;

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    const EXCLUDED_NAMESPACES: &[&str] = &["wasi"];

    let meta = component.component_type();
    println!("imports:");
    for (name, item) in meta.imports(&engine) {
        print_as_yaml(&engine, name, &item, "  ".to_owned(), EXCLUDED_NAMESPACES);
    }
    println!("exports:");
    for (name, item) in meta.exports(&engine) {
        print_as_yaml(&engine, name, &item, "  ".to_owned(), EXCLUDED_NAMESPACES);
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

fn print_as_yaml(
    engine: &Engine,
    name: &str,
    item: &ComponentItem,
    mut indent: String,
    excluded_namespaces: &[&str],
) {
    match name.split_once(':') {
        Some((lhs, _)) if excluded_namespaces.contains(&lhs) => return,
        _ => {}
    }

    println!("{indent}- name: {name}");
    indent.push_str("  ");

    match item {
        ComponentItem::ComponentInstance(v) => {
            println!("{indent}component-instance:");
            indent.push_str("  ");
            println!("{indent}exports:");
            indent.push_str("  ");
            for (name, item) in v.exports(engine) {
                print_as_yaml(engine, name, &item, indent.clone(), excluded_namespaces)
            }
        }
        ComponentItem::ComponentFunc(v) => {
            println!("{indent}component-func:");
            indent.push_str("  ");

            let params = v.params();
            if params.len() != 0 {
                println!("{indent}params:");
                for vv in params {
                    println!("{indent}  - {vv:?}");
                }
            } else {
                println!("{indent}params: []");
            }

            let results = v.results();
            if results.len() != 0 {
                println!("{indent}results:");
                for vv in results {
                    println!("{indent}  - {vv:?}");
                }
            } else {
                println!("{indent}results: []");
            }
        }
        v => unimplemented!("{v:?}"),
    }
}
