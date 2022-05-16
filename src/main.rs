use anyhow::Result;
use persy::{Config, Persy, TransactionConfig, ValueMode};
use rand::Rng;
use std::{
  env,
  fs::{remove_dir_all, remove_file},
  mem::MaybeUninit,
};

#[allow(non_upper_case_globals)]
pub mod index {
  pub const test: &str = "test";
}

pub fn run<const N: usize>() -> Result<()> {
  let dir = env::temp_dir();
  let mut rng = rand::thread_rng();
  let mut li: [[u64; 2]; N] = unsafe { MaybeUninit::uninit().assume_init() };
  for i in 0..N {
    li[i][0] = rng.gen();
    li[i][1] = rng.gen();
  }

  {
    let filename = "persy";
    let dbpath = dir.join(filename);

    remove_file(&dbpath)?;

    let tx_config = TransactionConfig::new().set_background_sync(true);

    let db = Persy::open_or_create_with(dbpath, Config::new(), |db| {
      let mut tx = db.begin()?;
      //tx.create_segment("test")?;
      tx.create_index::<u64, u64>(index::test, ValueMode::Replace)?;
      tx.prepare()?.commit()?;
      Ok(())
    })?;
    {
      for [k, v] in li {
        let mut tx = db.begin_with(tx_config.clone())?;
        tx.put(index::test, k, v)?;
        tx.prepare()?.commit()?;
      }
    }
  }

  Ok(())
}

fn main() -> Result<()> {
  run::<20000>()
}
