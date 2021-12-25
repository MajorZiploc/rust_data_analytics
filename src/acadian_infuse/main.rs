use crate::utils::main::*;
use polars::prelude::*;
use std::borrow::Cow;
use std::io::{Error, ErrorKind};

pub fn run() -> Result<()> {
  let config = get_config(&"acadian_infuse".to_string())?;
  dbg!(&config);
  let data_key = "data";
  let data_path = config
    .file_locations
    .get(data_key)
    //.ok_or(PolarsError::InvalidOperation(Cow::from(format!("{} not found in file locations map", &data_key))))?;
    .ok_or(PolarsError::Io(Error::new(ErrorKind::NotFound, "key not found in file locations map")))?;
  let df = CsvReader::from_path(data_path)?.infer_schema(None).has_header(true).finish()?;
  let rm_dashes = df.column("lines_added")?.neq("-");
  let mut df = df.filter(&rm_dashes)?;
  // TODO: make a helper to change a column
  let x = df.column("lines_added")?.cast(&DataType::UInt32)? * 2; // .i32()?;
  df.drop_in_place("lines_added")?;
  df = df.hstack(&[x])?;

  dbg!(&df);
  dbg!(df.dtypes());
  Ok(())
}
