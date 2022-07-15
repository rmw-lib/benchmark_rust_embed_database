写了一点代码测试了下各个数据库和字典多线程下的性能

写入 / 读取 1 万个 u64-u64 的键值对

测试机器 :

* MacBook Pro (Retina, 15-inch, Mid 2015)
* 2.2 GHz 四核 Intel Core i7
* 16 GB 1600 MHz DDR3
* HS-SSD-C2000Pro 2048G

没加 mdbx、lmdb 的测试，因为我感觉这种数据库在手机上数据一大就会被杀掉，尤其是苹果的手机 ( [Блеск и нищета key-value базы данных LMDB в приложениях для iOS](https://habr.com/ru/company/vk/blog/480850/) )。

# yakv
* insert 2.905 万次/秒
* get 73.532 万次/秒

# rusty_leveldb
* insert 57.543 万次/秒
* get 76.772 万次/秒

# duckdb
* insert 0.507 万次/秒
* get 1.818 万次/秒

# rocksdb
* insert 13.534 万次/秒
* get 339.733 万次/秒

# persy
* insert 0.403 万次/秒
* get 5.567 万次/秒

# sled
* insert 52.718 万次/秒
* get 302.927 万次/秒

# BTreeMap<u64, u64>
* insert 262.636 万次/秒
* get 496.465 万次/秒

# HashMap<u64, u64>
* insert 513.991 万次/秒
* get 725.067 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 197.207 万次/秒
* get 400.919 万次/秒

# dashmap
* insert 1136.235 万次/秒
* get 1658.466 万次/秒
