写了一点代码测试了下各个数据库和字典多线程下的性能

写入 / 读取 1 万个 u64-u64 的键值对

测试机器 :

* MacBook Pro (Retina, 15-inch, Mid 2015)
* 2.2 GHz 四核 Intel Core i7
* 16 GB 1600 MHz DDR3
* HS-SSD-C2000Pro 2048G

没加 mdbx、lmdb 的测试，因为我感觉这种数据库在手机上数据一大就会被杀掉，尤其是苹果的手机 ( [Блеск и нищета key-value базы данных LMDB в приложениях для iOS](https://habr.com/ru/company/vk/blog/480850/) )。

# yakv
* insert 3.259 万次/秒
* get 74.440 万次/秒

# rusty_leveldb
* insert 64.710 万次/秒
* get 80.860 万次/秒

# duckdb
* insert 0.794 万次/秒
* get 2.402 万次/秒

# rocksdb
* insert 14.632 万次/秒
* get 375.488 万次/秒

# persy
* insert 0.425 万次/秒
* get 6.028 万次/秒

# sled
* insert 39.807 万次/秒
* get 229.160 万次/秒

# BTreeMap<u64, u64>
* insert 222.473 万次/秒
* get 461.114 万次/秒

# HashMap<u64, u64>
* insert 342.796 万次/秒
* get 305.308 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 153.272 万次/秒
* get 401.685 万次/秒

# dashmap
* insert 1573.386 万次/秒
* get 1978.905 万次/秒
