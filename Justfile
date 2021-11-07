# vim: filetype=bash

install:
  rustup component add rustfmt

format:
  cargo fmt

run:
  cargo run

build:
  cargo build

test:
  cargo test

