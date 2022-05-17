写了一点代码测试了下各个数据库和字典多线程下的性能

写入 / 读取 1 万个 u64-u64 的键值对

测试机器 :

* MacBook Pro (Retina, 15-inch, Mid 2015)
* 2.2 GHz 四核 Intel Core i7
* 16 GB 1600 MHz DDR3
* HS-SSD-C2000Pro 2048G

没加 mdbx、lmdb 的测试，因为我感觉这种数据库在手机上数据一大就会被杀掉，尤其是苹果的手机 ( [Блеск и нищета key-value базы данных LMDB в приложениях для iOS](https://habr.com/ru/company/vk/blog/480850/) )。

# yakv
* insert 3.475 万次/秒
* get 65.239 万次/秒

# rusty_leveldb
* insert 56.417 万次/秒
* get 73.920 万次/秒

# duckdb
* insert 0.813 万次/秒
* get 2.217 万次/秒

# rocksdb
* insert 23.443 万次/秒
* get 398.987 万次/秒

# persy
* insert 0.420 万次/秒
* get 5.891 万次/秒

# sled
* insert 50.613 万次/秒
* get 417.499 万次/秒

# BTreeMap<u64, u64>
* insert 226.945 万次/秒
* get 463.239 万次/秒

# HashMap<u64, u64>
* insert 219.388 万次/秒
* get 399.617 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 155.332 万次/秒
* get 372.324 万次/秒

# dashmap
* insert 875.123 万次/秒
* get 1046.531 万次/秒
