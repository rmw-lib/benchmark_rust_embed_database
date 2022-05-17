写了一点代码测试了下各个数据库和字典多线程下的性能

写入 / 读取 1 万个 u64-u64 的键值对

测试机器 :

* MacBook Pro (Retina, 15-inch, Mid 2015)
* 2.2 GHz 四核 Intel Core i7
* 16 GB 1600 MHz DDR3
* HS-SSD-C2000Pro 2048G

没加 mdbx、lmdb 的测试，因为我感觉这种数据库在手机上数据一大就会被杀掉，尤其是苹果的手机 ( [Блеск и нищета key-value базы данных LMDB в приложениях для iOS](https://habr.com/ru/company/vk/blog/480850/) )。

# yakv
* insert 3.448 万次 / 秒
* get 105.099 万次 / 秒

# rusty_leveldb
* insert 67.585 万次 / 秒
* get 68.402 万次 / 秒

# duckdb
* insert 0.760 万次 / 秒
* get 2.164 万次 / 秒

# rocksdb
* insert 21.017 万次 / 秒
* get 437.990 万次 / 秒

# persy
* insert 0.429 万次 / 秒
* get 5.699 万次 / 秒

# sled
* insert 38.291 万次 / 秒
* get 404.963 万次 / 秒

# BTreeMap<u64, u64>
* insert 294.752 万次 / 秒
* get 219.141 万次 / 秒

# HashMap<u64, u64>
* insert 576.472 万次 / 秒
* get 1046.491 万次 / 秒

# btree_slab::BTreeMap<u64, u64>
* insert 287.588 万次 / 秒
* get 529.691 万次 / 秒

# dashmap
* insert 977.125 万次 / 秒
* get 1570.680 万次 / 秒
