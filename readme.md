写了一点代码测试了下各个数据库和字典多线程下的性能

写入 / 读取 1 万个 u64-u64 的键值对

测试机器 :

* MacBook Pro (Retina, 15-inch, Mid 2015)
* 2.2 GHz 四核 Intel Core i7
* 16 GB 1600 MHz DDR3
* HS-SSD-C2000Pro 2048G

没加 mdbx、lmdb 的测试，因为我感觉这种数据库在手机上数据一大就会被杀掉，尤其是苹果的手机 ( [Блеск и нищета key-value базы данных LMDB в приложениях для iOS](https://habr.com/ru/company/vk/blog/480850/) )。

# yakv
* insert 3.360 万次/秒
* get 72.278 万次/秒

# rusty_leveldb
* insert 68.712 万次/秒
* get 78.862 万次/秒

# duckdb
* insert 0.786 万次/秒
* get 2.318 万次/秒

# rocksdb
* insert 22.670 万次/秒
* get 412.303 万次/秒

# persy
* insert 0.418 万次/秒
* get 6.061 万次/秒

# sled
* insert 46.019 万次/秒
* get 378.839 万次/秒

# BTreeMap<u64, u64>
* insert 146.547 万次/秒
* get 469.926 万次/秒

# HashMap<u64, u64>
* insert 412.386 万次/秒
* get 456.783 万次/秒

# btree_slab::BTreeMap<u64, u64>
* insert 141.745 万次/秒
* get 256.415 万次/秒

# dashmap
* insert 1174.819 万次/秒
* get 1324.761 万次/秒
