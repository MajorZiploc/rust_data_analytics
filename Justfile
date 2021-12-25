# vim: filetype=bash

run:
  RUSTFLAGS="-Awarnings" cargo run;

run-module MODULE_PATH:
  #!/usr/bin/env bash
  . ~/.bashrc;
  set -euo pipefail
  module=`basename {{MODULE_PATH}}`;
  sed -E -i'' "s/(use crate::)\w+?(::main::\*; \/\/ module to run)/\1$module\2/" "./src/main.rs";
  sed -E -i'' "s/(mod )\w+(; \/\/ module to run)/\1$module\2/" "./src/main.rs";
  RUSTFLAGS="-Awarnings" cargo run;

install:
  rustup component add rustfmt

format:
  cargo fmt

build:
  cargo build

test:
  RUSTFLAGS="-Awarnings" cargo test

