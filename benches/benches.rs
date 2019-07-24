#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn bench_get_json_records_function(b: &mut Bencher) {
    oxicsv::set_records();

    b.iter(|| oxicsv::get_json_records());
}

#[bench]
fn bench_get_records_static(b: &mut Bencher) {
    oxicsv::set_records();

    b.iter(|| oxicsv::get_records());
}
