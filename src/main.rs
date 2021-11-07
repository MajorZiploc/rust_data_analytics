use crate::example::main::*;

mod example;

fn main() {
  let result = run();
  if let Err(err) = result {
    dbg!(err);
  };
}
