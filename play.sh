#!/bin/bash

repo_tag=sammyne/wasm-studio-rs:1404091

name=github-wasm-studio-rs

docker stop $name

docker wait $name

docker run -td --rm                             \
  -e CARGO_HOME=/root/.cargo                    \
  --name $name                                  \
  -v $PWD:/workspace                            \
  -v $PWD/_cargo/registry:/root/.cargo/registry \
  -v $PWD/_cargo/git:/root/.cargo/git           \
  -w /workspace                                 \
  $repo_tag                                     \
    bash -c 'echo "export PATH=/root/.cargo/bin:$PATH" >> /root/.bashrc; bash'

if [ -f _git/gitconfig ]; then
  docker cp _git/gitconfig $name:/root/.gitconfig
fi

if [ -d _ssh ]; then
  docker cp _ssh $name:/root/.ssh
fi
