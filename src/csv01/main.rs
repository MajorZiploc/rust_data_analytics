use polars::prelude::*;
use std::collections::HashMap;
use std::env;
use std::io;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};

struct Config {
  file_locations: HashMap<String, String>
}

fn get_config() -> std::result::Result<PathBuf, Error> {
  let exe = env::current_exe()?;
  let project_root = exe.parent()
    .and_then(|p| p.parent())
    .and_then(|p| p.parent())
    .ok_or(Error::new(ErrorKind::Other, "could not find root of project"))?;
  dbg!(&project_root);
  Ok(project_root.join("data"))
}

pub fn run() -> Result<()> {
  let path = get_config()?;
  let df = CsvReader::from_path(path.join("data.csv"))?
    .infer_schema(None)
    .has_header(true)
    .finish()?
    ;
  dbg!(&df);
  Ok(())
}
