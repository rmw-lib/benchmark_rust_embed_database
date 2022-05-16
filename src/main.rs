use anyhow::Result;
use persy::{Config, Persy, ValueMode};
use std::env;

pub mod index {
  const test: String = "test";
}

fn main() -> Result<()> {
  let dir = env::temp_dir();

  let filename = "persy";
  Persy::open_or_create_with(dir.join(filename), Config::new(), |p| {
    let mut tx = p.begin()?;
    //tx.create_segment("test")?;
    tx.create_index::<u64, u64>(index::test, ValueMode::Replace)?;
    tx.prepare()?.commit()?;
    Ok(())
  })?;

  OK(())
}
