#![feature(test)]

extern crate lazy_static;
extern crate test;

use lazy_static::lazy_static;
use parallel_sum::*;
use test::Bencher;

lazy_static! {
    static ref SMALL_ARRAY: Vec<u8> = vec![1; 100];
    static ref LARGE_ARRAY: Vec<u8> = vec![1; 1_000_000_000];
}

#[bench]
fn bench_linear_small(bencher: &mut Bencher) {
    assert_eq!(1_u8, *SMALL_ARRAY.get(0).unwrap()); // access the array to force initialization
    bencher.iter(|| linear_sum(&SMALL_ARRAY));
}

#[bench]
fn bench_linear_large(bencher: &mut Bencher) {
    assert_eq!(1_u8, *LARGE_ARRAY.get(0).unwrap());
    bencher.iter(|| linear_sum(&LARGE_ARRAY));
}

#[bench]
fn bench_threaded_small(bencher: &mut Bencher) {
    assert_eq!(1_u8, *SMALL_ARRAY.get(0).unwrap());
    bencher.iter(|| threaded_sum(&SMALL_ARRAY));
}

#[bench]
fn bench_threaded_large(bencher: &mut Bencher) {
    assert_eq!(1_u8, *LARGE_ARRAY.get(0).unwrap());
    bencher.iter(|| threaded_sum(&LARGE_ARRAY));
}
