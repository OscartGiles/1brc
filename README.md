
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
