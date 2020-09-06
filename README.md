# Erasure code

This is an implementation of reed-solomon code.

## Run

```bash
git clone https://github.com/OmarElawady/reed-solomon.git
cd reed-solomon/sol_storage/
cargo build
./target/debug/sol_storage
```

## Sample

The result of the previous sample should be the following:

```
DATA   : [[1], [2], [3]]
ENCODED: [[1], [2], [3], [0], [21]]
Rcovering using the subset [[2], [3], [21]]
DECODED: [[1], [2], [3]]
```

Which means that we had originaly 3 bytes 1, 2, and 3 sharded into 3 parts. Those three parts were augmented by two parity bytes. The original data can be recovered using any three bytes of the five. Here I used the second, third and fifth byte. For a file of length divisible by three (padding with zero if necessary), it should be divided into three equal parts, each part occupies a row. The result of the encoding is the augmentation with a number of parity blocks of size equal to any part of them. The number of parity blocks and data shards are parameters passed to the algorithm.
