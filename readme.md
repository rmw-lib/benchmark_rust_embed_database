
# yakv
* insert 4.060 万次/秒
* get 109.638 万次/秒

# rusty_leveldb
* insert 63.247 万次/秒
* get 78.607 万次/秒

# duckdb
* insert 0.689 万次/秒
* get 2.339 万次/秒

# rocksdb
* insert 16.708 万次/秒
* get 438.628 万次/秒

# persy
* insert 0.386 万次/秒
* get 5.169 万次/秒

# sled
* insert 43.332 万次/秒
* get 232.912 万次/秒

# BTreeMap<u64, u64>
* insert 183.795 万次/秒
* get 568.840 万次/秒

# HashMap<u64, u64>
* insert 316.936 万次/秒
* get 440.708 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 151.855 万次/秒
* get 321.035 万次/秒

# dashmap
* insert 1501.522 万次/秒
* get 2232.312 万次/秒
