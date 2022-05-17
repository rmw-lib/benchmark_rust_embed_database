写了一点代码测试了下各个数据库和字典多线程下的性能

写入 / 读取 1 万个 u64-u64 的键值对

测试机器 :

* MacBook Pro (Retina, 15-inch, Mid 2015)
* 2.2 GHz 四核 Intel Core i7
* 16 GB 1600 MHz DDR3
* HS-SSD-C2000Pro 2048G

没加 mdbx、lmdb 的测试，因为我感觉这种数据库在手机上数据一大就会被杀掉，尤其是苹果的手机 ( [Блеск и нищета key-value базы данных LMDB в приложениях для iOS](https://habr.com/ru/company/vk/blog/480850/) )。

# yakv
* insert 3.337 万次/秒
* get 65.069 万次/秒

# rusty_leveldb
* insert 72.011 万次/秒
* get 89.962 万次/秒

# duckdb
* insert 0.745 万次/秒
* get 2.373 万次/秒

# rocksdb
* insert 20.021 万次/秒
* get 411.522 万次/秒

# persy
* insert 0.467 万次/秒
* get 6.042 万次/秒

# sled
* insert 51.904 万次/秒
* get 426.553 万次/秒

# BTreeMap<u64, u64>
* insert 122.171 万次/秒
* get 336.931 万次/秒

# HashMap<u64, u64>
* insert 335.859 万次/秒
* get 452.713 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 138.566 万次/秒
* get 274.748 万次/秒

# dashmap
* insert 1817.878 万次/秒
* get 2458.664 万次/秒
