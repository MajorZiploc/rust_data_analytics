use crate::csv01::main::*; // module to run
mod csv01; // module to run

fn main() {
  let result = run();
  if let Err(err) = result {
    dbg!(err);
  };
}
