# Run the functionnal (fast) tests only
```
$ cargo test --lib
```

# Run the benchmarking tests only
```
$ cargo bench --tests -j 1
```

# Benchmarking results

With a 3.40GHz i5-3570K (4 cores)

```
test bench_linear_large   ... bench: 374,919,052 ns/iter (+/- 1,569,592)
test bench_linear_small   ... bench:          39 ns/iter (+/- 2)
test bench_rayon_large    ... bench: 101,799,350 ns/iter (+/- 5,603,216)
test bench_rayon_small    ... bench:       7,150 ns/iter (+/- 1,020)
test bench_threaded_large ... bench: 102,608,831 ns/iter (+/- 10,054,809)
test bench_threaded_small ... bench:      77,260 ns/iter (+/- 28,839)
```
