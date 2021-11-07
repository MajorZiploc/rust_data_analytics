use polars::prelude::*;

pub fn run() -> Result<()> {
  let s0 = Series::new("a", &[1i64, 2, 3]);
  let s1 = Series::new("b", &[1i64, 1, 1]);
  let s2 = Series::new("c", &[2i64, 2, 2]);
  // construct a new ListChunked for a slice of Series.
  let list = Series::new("foo", &[s0, s1, s2]);

  // construct a few more Series.
  let s0 = Series::new("B", [1, 2, 3]);
  let s1 = Series::new("C", [1, 1, 1]);
  let s2 = Series::new("D", [1, 2, 3]);
  let s3 = Series::new("E", [1, 2, 3]);
  let mut df = DataFrame::new(vec![list, s0, s1, s2, s3])?;
  dbg!(&df);
  for col_name in vec!["C", "E"] {
    df.drop_in_place(col_name)?;
  }

  let exploded_df = df.explode("foo")?;
  dbg!(&exploded_df);
  Ok(())
}

