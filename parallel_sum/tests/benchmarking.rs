#![feature(test)]

extern crate lazy_static;
extern crate test;

use lazy_static::lazy_static;
use parallel_sum::{linear_sum, threaded_sum};
use test::Bencher;

lazy_static! {
    static ref SMALL_ARRAY: Vec<u8> = vec![1; 100];
    static ref LARGE_ARRAY: Vec<u8> = vec![1; 1_000_000_000];
}

fn bench_small(bencher: &mut Bencher, sum: fn(&'static [u8]) -> u64) {
    assert_eq!(1_u8, *SMALL_ARRAY.get(0).unwrap()); // access the array to trigger lazy initialization
    bencher.iter(|| sum(&SMALL_ARRAY));
}

fn bench_large(bencher: &mut Bencher, sum: fn(&'static [u8]) -> u64) {
    assert_eq!(1_u8, *LARGE_ARRAY.get(0).unwrap()); // access the array to trigger lazy initialization
    bencher.iter(|| sum(&LARGE_ARRAY));
}

#[bench]
fn bench_linear_small(bencher: &mut Bencher) {
    bench_small(bencher, linear_sum);
}

#[bench]
fn bench_linear_large(bencher: &mut Bencher) {
    bench_large(bencher, linear_sum);
}

#[bench]
fn bench_threaded_small(bencher: &mut Bencher) {
    bench_small(bencher, threaded_sum);
}

#[bench]
fn bench_threaded_large(bencher: &mut Bencher) {
    bench_large(bencher, threaded_sum);
}
