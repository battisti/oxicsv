#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn bench_get_records_function(b: &mut Bencher) {
    b.iter(|| oxicsv::get_records());
}

#[bench]
fn bench_get_records_static(b: &mut Bencher) {
    b.iter(|| &oxicsv::RECORDS);
}
