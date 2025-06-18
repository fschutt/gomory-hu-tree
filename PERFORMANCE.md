# ghct-rs performance

Run on a Macbook Air (M2).

```
Running benches/gomory_hu_benchmarks.rs (target/release/deps/gomory_hu_benchmarks-9f916fe2f413bd67)
Gnuplot not found, using plotters backend
Gomory-Hu Construction/Dense Random/10
                        time:   [30.074 µs 30.579 µs 31.392 µs]
                        thrpt:  [318.55 Kelem/s 327.02 Kelem/s 332.51 Kelem/s]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe
Gomory-Hu Construction/Dense Random/20
                        time:   [189.42 µs 193.96 µs 199.56 µs]
                        thrpt:  [100.22 Kelem/s 103.11 Kelem/s 105.59 Kelem/s]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
Gomory-Hu Construction/Dense Random/30
                        time:   [595.31 µs 600.78 µs 608.44 µs]
                        thrpt:  [49.307 Kelem/s 49.935 Kelem/s 50.394 Kelem/s]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
Gomory-Hu Construction/Sparse Random/10
                        time:   [4.0484 µs 4.0879 µs 4.1449 µs]
                        thrpt:  [2.4126 Melem/s 2.4462 Melem/s 2.4701 Melem/s]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
Gomory-Hu Construction/Sparse Random/30
                        time:   [161.89 µs 162.55 µs 163.31 µs]
                        thrpt:  [183.69 Kelem/s 184.56 Kelem/s 185.31 Kelem/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Gomory-Hu Construction/Sparse Random/50
                        time:   [744.28 µs 751.43 µs 759.63 µs]
                        thrpt:  [65.821 Kelem/s 66.540 Kelem/s 67.179 Kelem/s]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
Gomory-Hu Construction/Sparse Random/70
                        time:   [1.9618 ms 1.9757 ms 1.9985 ms]
                        thrpt:  [35.026 Kelem/s 35.431 Kelem/s 35.681 Kelem/s]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Gomory-Hu Construction/Grid Graph/9
                        time:   [13.528 µs 13.753 µs 14.077 µs]
                        thrpt:  [639.35 Kelem/s 654.40 Kelem/s 665.28 Kelem/s]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe
Gomory-Hu Construction/Grid Graph/25
                        time:   [108.56 µs 109.13 µs 109.70 µs]
                        thrpt:  [227.89 Kelem/s 229.08 Kelem/s 230.29 Kelem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
Gomory-Hu Construction/Grid Graph/49
                        time:   [505.34 µs 507.44 µs 509.94 µs]
                        thrpt:  [96.089 Kelem/s 96.563 Kelem/s 96.965 Kelem/s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

Min-Cut Queries from Gomory-Hu Tree/Random Queries/30
                        time:   [615.19 µs 615.90 µs 616.64 µs]
                        thrpt:  [162.17 Kelem/s 162.36 Kelem/s 162.55 Kelem/s]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
Benchmarking Min-Cut Queries from Gomory-Hu Tree/Random Queries/50: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.3s, enable flat sampling, or reduce sample count to 60.
Min-Cut Queries from Gomory-Hu Tree/Random Queries/50
                        time:   [986.55 µs 988.42 µs 990.26 µs]
                        thrpt:  [100.98 Kelem/s 101.17 Kelem/s 101.36 Kelem/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
Benchmarking Min-Cut Queries from Gomory-Hu Tree/Random Queries/100: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.3s, enable flat sampling, or reduce sample count to 50.
Min-Cut Queries from Gomory-Hu Tree/Random Queries/100
                        time:   [1.8055 ms 1.8080 ms 1.8106 ms]
                        thrpt:  [55.230 Kelem/s 55.311 Kelem/s 55.387 Kelem/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

```