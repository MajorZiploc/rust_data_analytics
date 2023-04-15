export JUST_PROJECT_ROOT="$(pwd)";

function just_run {
  RUSTFLAGS="-Awarnings -Cembed-bitcode=no" cargo run;
}

function just_run_module {
  local module_path="${1:?}";
  module=`basename "$module_path"`;
  sed -E -i'' "s/(use crate::)\w+?(::main::\*; \/\/ module to run)/\1$module\2/" "./src/main.rs";
  sed -E -i'' "s/(mod )\w+(; \/\/ module to run)/\1$module\2/" "./src/main.rs";
  RUSTFLAGS="-Awarnings" cargo run;
}

function just_install {
  rustup component add rustfmt;
}

function just_format {
  cargo fmt;
}

function just_build {
  cargo build;
}

function just_clean {
  cargo clean;
}

function just_test {
  RUSTFLAGS="-Awarnings" cargo test;
}

