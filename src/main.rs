#![feature(try_trait_v2)]

use anyhow::Result;
use parking_lot::Mutex;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::{
  collections::{BTreeMap, HashMap},
  env,
  fs::File,
  fs::{remove_dir_all, remove_file},
  io::Write,
  mem::MaybeUninit,
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
        "{} {} rec/s",
        stringify!($op),
        (1000.0 * N as f64 / elapsed.as_millis() as f64) as u64
      );
      Ok::<_, anyhow::Error>(())
    }?};
  }

  {
    let filename = "rocksdb";
    println!("\n{filename}");
    let dbpath = dir.join(filename);
    let _ = remove_dir_all(&dbpath);
    let mut opt = rocksdb::Options::default();
    opt.create_if_missing(true);
    opt.set_max_open_files(10000);
    opt.set_use_fsync(false);
    opt.set_bytes_per_sync(8388608);
    opt.optimize_for_point_lookup(1024);
    opt.set_table_cache_num_shard_bits(6);
    opt.set_max_write_buffer_number(32);
    opt.set_write_buffer_size(536870912);
    opt.set_target_file_size_base(1073741824);
    opt.set_min_write_buffer_number_to_merge(4);
    opt.set_level_zero_stop_writes_trigger(2000);
    opt.set_level_zero_slowdown_writes_trigger(0);
    opt.set_compaction_style(rocksdb::DBCompactionStyle::Universal);
    opt.set_max_background_jobs(4);
    opt.set_disable_auto_compactions(true);
    let db = Arc::new(rocksdb::DB::open(&opt, dbpath)?);

    elapsed!(insert, |kv| -> Result<()> {
      let [k, v] = kv;
      db.put(&k.to_be_bytes(), &v.to_le_bytes())?;
      Ok(())
    });

    let file = Arc::new(Mutex::new(File::create(dir.join("out"))?));
    elapsed!(get, |kv| -> Result<()> {
      let [k, _] = kv;
      if let Some(i) = db.get_pinned(&k.to_be_bytes())? {
        file.lock().write_all(&i)?;
      }
      Ok(())
    });
  }

  {
    use persy::{Config, Persy, TransactionConfig, ValueMode};
    let filename = "persy";
    println!("\n{filename}");
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
    println!("\n{filename}");
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
        file.lock().write_all(&i.as_ref())?;
      }
      Ok(())
    });
  }

  macro_rules! map {
    ($name:ident) => {
      println!(concat!("\n", stringify!($name)));
      let db = Arc::new(Mutex::new($name::new()));

      elapsed!(insert, |kv| -> Result<()> {
        let [k, v] = kv;
        db.lock().insert(k, v);
        Ok(())
      });

      let file = Arc::new(Mutex::new(File::create(dir.join("out"))?));
      elapsed!(get, |kv| -> Result<()> {
        let [k, _] = kv;
        if let Some(i) = db.lock().get(&k) {
          file.lock().write_all(&i.to_le_bytes())?;
        }
        Ok(())
      });
    };
  }

  map!(BTreeMap);
  map!(HashMap);

  Ok(())
}

fn main() -> Result<()> {
  run::<10000>()
}
