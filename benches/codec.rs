use criterion::criterion_main;
use criterion::Criterion;
use criterion::{black_box, criterion_group};
use rand::Rng;
use rayon::prelude::*;
use spartan_codec::Spartan256bit4096;
use std::time::{Duration, Instant};

fn random_bytes<const BYTES: usize>() -> [u8; BYTES] {
    let mut bytes = [0u8; BYTES];
    rand::thread_rng().fill(&mut bytes[..]);
    bytes
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Spartan-codec");
    group.sample_size(500);
    group.measurement_time(Duration::from_secs(30));

    let genesis_piece = random_bytes();
    let encoding_key_hash = random_bytes::<32>();
    let nonce = rand::random();

    let spartan = Spartan256bit4096::new(genesis_piece);

    group.bench_function("Encode-single", |b| {
        b.iter(|| {
            black_box(spartan.encode(encoding_key_hash, nonce, 1));
        })
    });

    group.bench_function("Encode-parallel", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();

            black_box((0..iters).into_par_iter().for_each(|i| {
                spartan.encode(encoding_key_hash, i, 1);
            }));

            start.elapsed()
        })
    });

    let encoding = spartan.encode(encoding_key_hash, nonce, 1);

    group.bench_function("Decode", |b| {
        b.iter(|| {
            black_box(spartan.is_valid(encoding, encoding_key_hash, nonce, 1));
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
