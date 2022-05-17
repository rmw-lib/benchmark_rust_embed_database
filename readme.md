写了一点代码测试了下各个数据库和字典多线程下的性能

写入 / 读取 1 万个 u64-u64 的键值对

测试机器 :

* MacBook Pro (Retina, 15-inch, Mid 2015)
* 2.2 GHz 四核 Intel Core i7
* 16 GB 1600 MHz DDR3
* HS-SSD-C2000Pro 2048G

没加 mdbx、lmdb 的测试，因为我感觉这种数据库在手机上数据一大就会被杀掉，尤其是苹果的手机 ( [Блеск и нищета key-value базы данных LMDB в приложениях для iOS](https://habr.com/ru/company/vk/blog/480850/) )。

# yakv
* insert 3.350 万次/秒
* get 68.534 万次/秒

# rusty_leveldb
* insert 72.018 万次/秒
* get 92.076 万次/秒

# duckdb
* insert 0.787 万次/秒
* get 2.447 万次/秒

# rocksdb
* insert 14.198 万次/秒
* get 405.606 万次/秒

# persy
* insert 0.474 万次/秒
* get 6.185 万次/秒

# sled
* insert 57.240 万次/秒
* get 363.293 万次/秒

# BTreeMap<u64, u64>
* insert 128.813 万次/秒
* get 303.035 万次/秒

# HashMap<u64, u64>
* insert 280.975 万次/秒
* get 402.430 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 135.390 万次/秒
* get 253.015 万次/秒

# dashmap
* insert 1504.173 万次/秒
* get 2251.791 万次/秒
