use crate::example::main::*; // module to run
mod example; // module to run

fn main() {
  let result = run();
  if let Err(err) = result {
    dbg!(err);
  };
}
