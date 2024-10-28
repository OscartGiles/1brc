
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

## 5. Use memchar

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):     130.6 ms ±   5.3 ms    [User: 124.3 ms, System: 4.8 ms]
  Range (min … max):   127.1 ms … 152.6 ms    22 runs

Benchmark 1: ./target/release/brc_stackyak measurements_100_000_000.txt
  Time (mean ± σ):     12.282 s ±  0.348 s    [User: 11.773 s, System: 0.427 s]
  Range (min … max):   11.982 s … 12.970 s    10 runs

## Parallel

At first things were just as slow. Realised I had a print statement which locks stdout causing threads to block. Oppps.

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):      41.7 ms ±   1.0 ms    [User: 279.4 ms, System: 18.2 ms]
  Range (min … max):    40.0 ms …  45.0 ms    68 runs

Benchmark 1: ./target/release/brc_stackyak measurements_100_000_000.txt
  Time (mean ± σ):      3.410 s ±  0.136 s    [User: 37.013 s, System: 0.827 s]
  Range (min … max):    3.140 s …  3.586 s    10 runs

## Custom parser

Tests are failing because of rounding error

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):      29.6 ms ±   0.8 ms    [User: 145.4 ms, System: 16.3 ms]
  Range (min … max):    27.9 ms …  32.4 ms    89 runs

Benchmark 1: ./target/release/brc_stackyak measurements_100_000_000.txt
  Time (mean ± σ):      1.553 s ±  0.025 s    [User: 16.942 s, System: 0.713 s]
  Range (min … max):    1.526 s …  1.596 s    10 runs

## Inline stuff and configure build

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):      32.3 ms ±   0.7 ms    [User: 157.0 ms, System: 17.9 ms]
  Range (min … max):    31.0 ms …  35.1 ms    80 runs

Benchmark 1: ./target/release/brc_stackyak measurements_100_000_000.txt
  Time (mean ± σ):      1.464 s ±  0.090 s    [User: 15.782 s, System: 0.627 s]
  Range (min … max):    1.381 s …  1.665 s    10 runs

## Build with native

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):      32.3 ms ±   0.7 ms    [User: 154.1 ms, System: 18.2 ms]
  Range (min … max):    30.7 ms …  34.1 ms    81 runs

Benchmark 1: ./target/release/brc_stackyak measurements_100_000_000.txt
  Time (mean ± σ):      1.423 s ±  0.033 s    [User: 15.459 s, System: 0.633 s]
  Range (min … max):    1.393 s …  1.495 s    10 runs


## Custom memchar implementation using simd

Benchmark 1: ./target/release/brc_stackyak measurements_1_000_000.txt
  Time (mean ± σ):      27.4 ms ±   1.5 ms    [User: 126.6 ms, System: 16.5 ms]
  Range (min … max):    25.6 ms …  31.9 ms    83 runs

Benchmark 1: ./target/release/brc_stackyak measurements_100_000_000.txt
  Time (mean ± σ):      1.231 s ±  0.051 s    [User: 13.330 s, System: 0.578 s]
  Range (min … max):    1.164 s …  1.302 s    10 runs
