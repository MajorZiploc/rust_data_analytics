use polars::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
  file_locations: HashMap<String, String>,
}

fn set_full_paths(file_locations: HashMap<String, String>) -> std::result::Result<HashMap<String, String>, Error> {
  let project_root = get_project_root()?.join("data").join("csv01");
  let full_file_locations = file_locations
    .iter()
    .map(|fl| (fl.0.to_owned(), project_root.join(fl.1).to_str().unwrap_or("").to_owned()))
    .collect();
  Ok(full_file_locations)
}

fn get_project_root() -> std::result::Result<PathBuf, Error> {
  let exe = env::current_exe()?;
  let project_root = exe
    .parent()
    .and_then(|p| p.parent())
    .and_then(|p| p.parent())
    .ok_or(Error::new(ErrorKind::Other, "could not find root of project"))?;
  Ok(project_root.to_owned())
}

fn get_in_config() -> std::result::Result<Config, Error> {
  let project_root = get_project_root()?;
  let config_path = project_root.join("configs").join("csv01.json");
  let file = File::open(config_path)?;
  let in_config: Config = serde_json::from_reader(file)?;
  Ok(in_config)
}

fn get_config() -> std::result::Result<Config, Error> {
  let in_config = get_in_config()?;
  let file_locations = set_full_paths(in_config.file_locations)?;
  Ok(Config { file_locations })
}

pub fn run() -> Result<()> {
  let config = get_config()?;
  dbg!(&config);
  let data_path = config
    .file_locations
    .get("data")
    .ok_or(PolarsError::Io(Error::new(ErrorKind::NotFound, "key not found in file locations map")))?;
  let df = CsvReader::from_path(data_path)?.infer_schema(None).has_header(true).finish()?;
  dbg!(&df);
  Ok(())
}
