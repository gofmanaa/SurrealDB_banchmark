## SurrealDB benchmarks

This repository contains benchmarks for SurrealDB with a specific focus on insertion performance. It includes a comparison with Wherry, which has been observed to be slower on inserts in certain conditions. The goal of these benchmarks is to provide insights into the performance characteristics of SurrealDB and highlight areas for optimization.

## Installation

Clone the repository and install the required dependencies:
```
git clone https://github.com/gofmanaa/SurrealDB_banchmark.git
cd SurrealDB_banchmark
cargo build --release
```

Run SurrealDB server:

```
 docker run --rm --pull always -p 8000:8000  --user $(id -u) -v $(pwd)/mydata:/mydata surrealdb/surrealdb:latest start --log info --auth --user root --pass root file:/mydata/mydatabase.db
```

Run benchmark

```
 ./target/release/surrealDB_banchmarks
```


05/2024 Nothing change:

![X post](x_post.png) 