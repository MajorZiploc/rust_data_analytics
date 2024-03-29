use crate::utils::main::*;
use polars::prelude::*;
use std::io::{Error, ErrorKind};
use std::borrow::Cow;

pub fn run() -> Result<()> {
  let config = get_config(&"csv01".to_string())?;
  dbg!(&config);
  let data_key = "data";
  let data_path = config
    .file_locations
    .get(data_key)
    //.ok_or(PolarsError::InvalidOperation(Cow::from(format!("{} not found in file locations map", &data_key))))?;
    .ok_or(PolarsError::Io(Error::new(ErrorKind::NotFound, "key not found in file locations map")))?;
  let df = CsvReader::from_path(data_path)?.infer_schema(None).has_header(true).finish()?;
  dbg!(&df);
  Ok(())
}
