写了一点代码测试了下各个数据库和字典多线程下的性能

写入 / 读取 1 万个 u64-u64 的键值对

测试机器 :

* MacBook Pro (Retina, 15-inch, Mid 2015)
* 2.2 GHz 四核 Intel Core i7
* 16 GB 1600 MHz DDR3
* HS-SSD-C2000Pro 2048G

没加 mdbx、lmdb 的测试，因为我感觉这种数据库在手机上数据一大就会被杀掉，尤其是苹果的手机 ( [Блеск и нищета key-value базы данных LMDB в приложениях для iOS](https://habr.com/ru/company/vk/blog/480850/) )。

# yakv
* insert 3.443 万次/秒
* get 68.894 万次/秒

# rusty_leveldb
* insert 73.724 万次/秒
* get 89.141 万次/秒

# duckdb
* insert 0.739 万次/秒
* get 2.321 万次/秒

# rocksdb
* insert 22.327 万次/秒
* get 405.206 万次/秒

# persy
* insert 0.468 万次/秒
* get 6.158 万次/秒

# sled
* insert 54.027 万次/秒
* get 346.421 万次/秒

# BTreeMap<u64, u64>
* insert 138.095 万次/秒
* get 389.658 万次/秒

# HashMap<u64, u64>
* insert 324.526 万次/秒
* get 308.472 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 124.946 万次/秒
* get 293.728 万次/秒

# dashmap
* insert 1076.526 万次/秒
* get 1310.497 万次/秒
