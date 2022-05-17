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
* get 79.735 万次/秒

# rusty_leveldb
* insert 71.962 万次/秒
* get 80.651 万次/秒

# duckdb
* insert 0.802 万次/秒
* get 2.395 万次/秒

# rocksdb
* insert 14.504 万次/秒
* get 441.737 万次/秒

# persy
* insert 0.455 万次/秒
* get 6.256 万次/秒

# sled
* insert 54.152 万次/秒
* get 352.347 万次/秒

# BTreeMap<u64, u64>
* insert 135.659 万次/秒
* get 406.250 万次/秒

# HashMap<u64, u64>
* insert 381.223 万次/秒
* get 353.882 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 119.054 万次/秒
* get 309.330 万次/秒

# dashmap
* insert 1647.086 万次/秒
* get 2438.364 万次/秒
