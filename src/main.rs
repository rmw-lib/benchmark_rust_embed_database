#![feature(try_trait_v2)]

use anyhow::Result;
use parking_lot::Mutex;
use persy::{Config, Persy, TransactionConfig, ValueMode};
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::{
  env,
  fs::File,
  fs::{create_dir_all, remove_dir_all, remove_file},
  io::Write,
  mem::MaybeUninit,
  ops::Try,
  time::Instant,
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

  macro_rules! elapsed {
    ($op:ident, $func:expr) => {{
      let now = Instant::now();
      li.into_par_iter().try_for_each($func)?;
      let elapsed = now.elapsed();
      println!(
        "{} insert {:.2} rec/s",
        stringify!($op),
        1000.0 * N as f64 / elapsed.as_millis() as f64
      );
      Ok::<_, anyhow::Error>(())
    }?};
  }

  {
    let filename = "persy";
    println!("{filename}");
    let dbpath = dir.join(filename);
    let _ = remove_file(&dbpath);

    let tx_config = TransactionConfig::new().set_background_sync(true);

    let db = Persy::open_or_create_with(dbpath, Config::new(), |db| {
      let mut tx = db.begin()?;
      //tx.create_segment("test")?;
      tx.create_index::<u64, u64>(index::test, ValueMode::Replace)?;
      tx.prepare()?.commit()?;
      Ok(())
    })?;
    elapsed!(insert, |kv| -> Result<()> {
      let [k, v] = kv;
      let mut tx = db.begin_with(tx_config.clone())?;
      tx.put(index::test, k, v)?;
      tx.prepare()?.commit()?;
      Ok(())
    });

    let file = Arc::new(Mutex::new(File::create(dir.join("out"))?));
    elapsed!(get, |kv| -> Result<()> {
      let [k, _] = kv;
      for (_, vli) in db.range::<u64, u64, _>(index::test, k..k + 1)? {
        for i in vli {
          file.lock().write_all(&i.to_le_bytes())?;
          break;
        }
        break;
      }
      Ok(())
    });
  }

  {
    let filename = "sled";
    println!("{filename}");
    let dbpath = dir.join(filename);
    let _ = remove_dir_all(&dbpath);

    let db = sled::open(dbpath)?;
    elapsed!(insert, |kv| -> Result<()> {
      let [k, v] = kv;
      db.insert(&k.to_be_bytes(), &v.to_le_bytes())?;
      Ok(())
    });
    let file = Arc::new(Mutex::new(File::create(dir.join("out"))?));
    elapsed!(get, |kv| -> Result<()> {
      let [k, _] = kv;
      if let Some(i) = db.get(&k.to_be_bytes())? {
        file.lock().write_all(i.as_ref().try_into().unwrap())?;
      }
      Ok(())
    });
  }
  Ok(())
}

fn main() -> Result<()> {
  run::<20000>()
}
