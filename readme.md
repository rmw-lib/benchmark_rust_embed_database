写了一点代码测试了下各个数据库和字典多线程下的性能

写入 / 读取 1 万个 u64-u64 的键值对

测试机器 :

* MacBook Pro (Retina, 15-inch, Mid 2015)
* 2.2 GHz 四核 Intel Core i7
* 16 GB 1600 MHz DDR3
* HS-SSD-C2000Pro 2048G

没加 mdbx、lmdb 的测试，因为我感觉这种数据库在手机上数据一大就会被杀掉，尤其是苹果的手机 ( [Блеск и нищета key-value базы данных LMDB в приложениях для iOS](https://habr.com/ru/company/vk/blog/480850/) )。

# yakv
* insert 4.608 万次/秒
* get 85.484 万次/秒

# rusty_leveldb
* insert 59.516 万次/秒
* get 74.670 万次/秒

# duckdb
* insert 0.625 万次/秒
* get 1.751 万次/秒

# rocksdb
* insert 14.342 万次/秒
* get 365.118 万次/秒

# persy
* insert 0.448 万次/秒
* get 4.852 万次/秒

# sled
* insert 49.088 万次/秒
* get 268.714 万次/秒

# BTreeMap<u64, u64>
* insert 193.827 万次/秒
* get 472.906 万次/秒

# HashMap<u64, u64>
* insert 398.714 万次/秒
* get 595.867 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 199.532 万次/秒
* get 341.224 万次/秒

# dashmap
* insert 1539.452 万次/秒
* get 2205.363 万次/秒
