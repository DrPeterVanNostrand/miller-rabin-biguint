use std::str::FromStr;

use num_bigint::BigUint;
use criterion::{criterion_group, criterion_main, Criterion};
use miller_rabin_biguint::miller_rabin_biguint;

fn miller_rabin_64_bit_prime(c: &mut Criterion) {
    let prime = BigUint::from(15930455692162817671u64);
    c.bench_function(
        "Miller-Rabin 64-bit",
        move |bencher| bencher.iter(|| miller_rabin_biguint(&prime))
    );
}

fn miller_rabin_255_bit_prime(c: &mut Criterion) {
    let prime = BigUint::from_str("57896044618658097711785492504343953926634992332820282019728792003956564819949").unwrap();
    c.bench_function(
        "Miller-Rabin 255-bit",
        move |bencher| bencher.iter(|| miller_rabin_biguint(&prime))
    );
}

criterion_group!{
    name = benches;
    config = Criterion::default();
    targets = miller_rabin_64_bit_prime, miller_rabin_255_bit_prime
}

criterion_main!(benches);
