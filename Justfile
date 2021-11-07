# vim: filetype=bash

install:
  rustup component add rustfmt

format:
  cargo fmt

run MODULE_PATH:
  #!/usr/bin/env bash
  . ~/.bashrc;
  set -euo pipefail
  module=`basename {{MODULE_PATH}}`;
  sed -E -i'' "s/(use crate::)\w+?(::main::\*; \/\/ module to run)/\1$module\2/" "./src/main.rs";
  sed -E -i'' "s/(mod )\w+(; \/\/ module to run)/\1$module\2/" "./src/main.rs";
  cargo run;

build:
  cargo build

test:
  cargo test

