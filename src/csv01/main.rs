use polars::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
  file_locations: HashMap<String, String>
}

fn get_config() -> std::result::Result<Config, Error> {
  let project_root = get_project_root()?;
  let config_path = project_root.join("configs").join("csv01.json");
  let file = File::open(config_path)?;
  let config: Config = serde_json::from_reader(file)?;
  Ok(config)
}

fn get_project_root() -> std::result::Result<Box<PathBuf>, Error> {
  let exe = env::current_exe()?;
  let project_root = exe.parent()
    .and_then(|p| p.parent())
    .and_then(|p| p.parent())
    .ok_or(Error::new(ErrorKind::Other, "could not find root of project"))?;
  Ok(Box::new(project_root.to_owned()))
}

pub fn run() -> Result<()> {
  let config = get_config()?;
  dbg!(config);
  let path = get_project_root()?;
  let df = CsvReader::from_path(path.join("data").join("csv01").join("data.csv"))?
    .infer_schema(None)
    .has_header(true)
    .finish()?
    ;
  dbg!(&df);
  Ok(())
}
