写了一点代码测试了下各个数据库和字典多线程下的性能

写入 / 读取 1 万个 u64-u64 的键值对

测试机器 :

* MacBook Pro (Retina, 15-inch, Mid 2015)
* 2.2 GHz 四核 Intel Core i7
* 16 GB 1600 MHz DDR3
* HS-SSD-C2000Pro 2048G

没加 mdbx、lmdb 的测试，因为我感觉这种数据库在手机上数据一大就会被杀掉，尤其是苹果的手机 ( [Блеск и нищета key-value базы данных LMDB в приложениях для iOS](https://habr.com/ru/company/vk/blog/480850/) )。

# yakv
* insert 3.357 万次/秒
* get 78.277 万次/秒

# rusty_leveldb
* insert 67.455 万次/秒
* get 82.439 万次/秒

# duckdb
* insert 0.791 万次/秒
* get 2.422 万次/秒

# rocksdb
* insert 13.668 万次/秒
* get 347.676 万次/秒

# persy
* insert 0.451 万次/秒
* get 6.243 万次/秒

# sled
* insert 57.031 万次/秒
* get 410.633 万次/秒

# BTreeMap<u64, u64>
* insert 130.212 万次/秒
* get 383.722 万次/秒

# HashMap<u64, u64>
* insert 333.025 万次/秒
* get 270.539 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 130.780 万次/秒
* get 302.349 万次/秒

# dashmap
* insert 1469.570 万次/秒
* get 1897.803 万次/秒
