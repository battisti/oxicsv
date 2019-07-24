#![feature(test)]

extern crate test;
use test::Bencher;

use oxicsv::get_json_records;

#[bench]
fn bench_get_json_records_function(b: &mut Bencher) {
    b.iter(|| {
        let _ = get_json_records();
    });
}
