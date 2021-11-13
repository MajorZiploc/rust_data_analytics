use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::result::Result as StdResult;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub file_locations: HashMap<String, String>,
}

fn get_project_root() -> StdResult<PathBuf, Error> {
  let exe = env::current_exe()?;
  let project_root = exe
    .parent()
    .and_then(|p| p.parent())
    .and_then(|p| p.parent())
    .ok_or(Error::new(ErrorKind::Other, "could not find root of project"))?;
  Ok(project_root.to_owned())
}

fn set_full_paths(
  file_locations: HashMap<String, String>,
  module_name: &String,
) -> StdResult<HashMap<String, String>, Error> {
  let project_root = get_project_root()?.join("data").join(module_name);
  let full_file_locations = file_locations
    .iter()
    .map(|fl| (fl.0.to_owned(), project_root.join(fl.1).to_str().unwrap_or("").to_owned()))
    .collect();
  Ok(full_file_locations)
}

fn get_in_config(module_name: &String) -> StdResult<Config, Error> {
  let project_root = get_project_root()?;
  let config_path = project_root.join("configs").join(module_name.clone() + ".json");
  let file = File::open(config_path)?;
  let in_config: Config = serde_json::from_reader(file)?;
  Ok(in_config)
}

pub fn get_config(module_name: &String) -> StdResult<Config, Error> {
  let in_config = get_in_config(module_name)?;
  let file_locations = set_full_paths(in_config.file_locations, module_name)?;
  Ok(Config { file_locations })
}
