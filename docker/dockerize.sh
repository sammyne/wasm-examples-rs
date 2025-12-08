#!/bin/bash

set -e

cd `dirname ${BASH_SOURCE[0]}`

repo_tag=sammyne/wasm-studio-rs:`git rev-parse --short HEAD`

build_arg_opts="--build-arg CARGO_COMPONENT_VERSION=0.21.1"
build_arg_opts="$build_arg_opts --build-arg MOLD_VERSION=2.40.4"
build_arg_opts="$build_arg_opts --build-arg WASM_TOOLS_VERSION=1.243.0"
build_arg_opts="$build_arg_opts --build-arg WASMTIME_VERSION=39.0.1"
build_arg_opts="$build_arg_opts --build-arg WIT_BINDGEN_VERSION=0.49.0"

docker build $build_arg_opts -t $repo_tag --progress plain .
