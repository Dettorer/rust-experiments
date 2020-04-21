#![feature(test)]
extern crate test;

use parallel_sum::linear_sum;
use test::Bencher;

#[bench]
fn bench_linear_small(bencher: &mut Bencher) {
    let array = vec![1; 100];
    bencher.iter(|| linear_sum(&array));
}

#[bench]
fn bench_linear_large(bencher: &mut Bencher) {
    let array = vec![1; 1000000000];
    bencher.iter(|| linear_sum(&array));
}
