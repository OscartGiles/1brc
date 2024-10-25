
```shell
cargo build --release
hyperfine --warmup 3 "./target/release/brc_stackyak measurements_1_000_000.txt"
```

## 1. Naive

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):     212.7 ms ±   0.9 ms    [User: 204.9 ms, System: 5.9 ms]
  Range (min … max):   211.7 ms … 215.3 ms    13 runs

Benchmark 1: ./target/release/brc_stackyak measurements_100_000_000.txt
    Time (mean ± σ):     19.956 s ±  0.143 s    [User: 19.529 s, System: 0.350 s]
    Range (min … max):   19.782 s … 20.216 s    10 runs

## 2. Change to FxHashMap

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):     205.5 ms ±   3.4 ms    [User: 197.5 ms, System: 6.1 ms]
  Range (min … max):   201.3 ms … 211.4 ms    13 runs

  measurements_100_000_000.txt
  Time (mean ± σ):     18.899 s ±  0.125 s    [User: 18.495 s, System: 0.343 s]
  Range (min … max):   18.749 s … 19.119 s    10 runs

## 3. Use mmap

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):     203.4 ms ±   2.0 ms    [User: 195.3 ms, System: 6.0 ms]
  Range (min … max):   201.0 ms … 208.7 ms    14 runs

Benchmark 1: ./target/release/brc_stackyak measurements_100_000_000.txt
  Time (mean ± σ):     18.894 s ±  0.266 s    [User: 18.462 s, System: 0.372 s]
  Range (min … max):   18.507 s … 19.306 s    10 runs

## 4. Use bytes

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):     202.3 ms ±   2.9 ms    [User: 195.0 ms, System: 5.5 ms]
  Range (min … max):   198.3 ms … 210.9 ms    14 runs

Benchmark 1: ./target/release/brc_stackyak measurements_100_000_000.txt
  Time (mean ± σ):     19.027 s ±  0.096 s    [User: 18.509 s, System: 0.452 s]
  Range (min … max):   18.919 s … 19.212 s    10 runs

## 4. Write to buffer

Write to a buffer for results rather than appending strings.

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):     134.4 ms ±   1.1 ms    [User: 128.0 ms, System: 4.7 ms]
  Range (min … max):   132.6 ms … 136.9 ms    21 runs

Benchmark 1: ./target/release/brc_stackyak measurements_100_000_000.txt
  Time (mean ± σ):     12.496 s ±  0.164 s    [User: 12.041 s, System: 0.418 s]
  Range (min … max):   12.321 s … 12.853 s    10 runs
