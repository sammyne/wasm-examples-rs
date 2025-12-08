FROM rust:1.91.1-slim-bookworm AS rust

ENV RUSTUP_DIST_SERVER="https://rsproxy.cn" \
  RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

RUN sed -i 's/deb.debian.org/mirrors.tencent.com/g' /etc/apt/sources.list.d/debian.sources  &&\
  sed -i 's|security.debian.org/debian-security|mirrors.tencent.com/debian-security|g'      \
    /etc/apt/sources.list.d/debian.sources

# libssl-dev 和 pkg-config 是 cargo-component 的依赖
# clang 是 mold 链接器的依赖
RUN apt update                                  &&\
  apt install -y clang libssl-dev pkg-config    &&\
  apt install -y git vim make                   &&\
  apt clean                                     &&\
  rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN rustup component add rustfmt && rustup target add wasm32-unknown-unknown wasm32-wasip1 wasm32-wasip2

#RUN echo 'export PATH=$CARGO_HOME/bin:$PATH' > /root/.bashrc


FROM rust AS cargo-bin

ARG CARGO_COMPONENT_VERSION
ARG WAC_VERSION
ARG WASM_TOOLS_VERSION
ARG WIT_BINDGEN_VERSION

RUN cargo install                           \
  cargo-component@$CARGO_COMPONENT_VERSION  \
  wac-cli@$WAC_VERSION                      \
  wasm-tools@$WASM_TOOLS_VERSION            \
  wit-bindgen-cli@$WIT_BINDGEN_VERSION

WORKDIR /output

RUN cp `which cargo-component` `which wac` `which wasm-tools` `which wit-bindgen` ./


FROM busybox:1.36.1 AS wasmtime

ARG WASMTIME_VERSION

ENV REPO_URL=https://github.com/bytecodealliance/wasmtime/releases/download/v$WASMTIME_VERSION

RUN wget $REPO_URL/wasmtime-v$WASMTIME_VERSION-x86_64-linux.tar.xz

RUN mkdir -p /output && tar -C /output --strip-components 1 -Jxvf wasmtime-v$WASMTIME_VERSION-x86_64-linux.tar.xz

WORKDIR /output/adapter-modules

RUN wget $REPO_URL/wasi_snapshot_preview1.command.wasm  &&\
  wget $REPO_URL/wasi_snapshot_preview1.proxy.wasm      &&\
  wget $REPO_URL/wasi_snapshot_preview1.reactor.wasm


FROM busybox:1.36.1 AS mold

ARG MOLD_VERSION

RUN wget https://github.com/rui314/mold/releases/download/v$MOLD_VERSION/mold-$MOLD_VERSION-x86_64-linux.tar.gz

RUN mkdir /output && tar -C /output --strip-components=1 -zxvf mold-$MOLD_VERSION-x86_64-linux.tar.gz


FROM rust

ENV PATH=/opt/wasmtime:/opt/mold/bin:$PATH

COPY --from=cargo-bin /output/* $CARGO_HOME/bin/

COPY --from=mold /output /opt/mold

COPY --from=wasmtime /output /opt/wasmtime
