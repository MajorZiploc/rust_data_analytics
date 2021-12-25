use crate::acadian_infuse::main::*; // module to run
mod acadian_infuse; // module to run
mod utils;

fn main() {
  let result = run();
  if let Err(err) = result {
    dbg!(err);
  };
}
