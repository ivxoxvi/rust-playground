use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use rand::{RngExt};

#[inline(never)]
fn add_if(v: u8, sum: &mut u64) {
    // cpu needs to predict this branch.
    if v >= 128 {
        *sum += v as u64;
    }
}

fn bench_predict_loop(arr: &[u8]) -> u64 {
    let mut sum = 0u64;
    for _ in 0u32..100000u32 {
        for &data in arr {
            add_if(data, &mut sum);
        }
    }
    std::hint::black_box(sum)
}

fn bench_branch(c: &mut Criterion) {
    let mut group = c.benchmark_group("branch_prediction");
    group.measurement_time(std::time::Duration::from_secs(6));
    group.sample_size(10);
    
    group.bench_function("good predict (sorted)", |b| {
        b.iter_batched(
            || {
                let mut arr = [0u8; 32768];
                let mut rng = rand::rng();
                for data in &mut arr {
                    *data = rng.random_range(0..255);
                }
                // This can help branch prediction.
                arr.sort();
                arr
            },
            |arr| bench_predict_loop(&arr),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("bad predict (unsorted)", |b| {
        b.iter_batched(
            || {
                let mut arr = [0u8; 32768];
                let mut rng = rand::rng();
                for data in &mut arr {
                    *data = rng.random_range(0..255);
                }
                // Unsorted, so branch prediction accuracy is roughly 50%.
                arr
            },
            |arr| bench_predict_loop(&arr),
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

criterion_group!(benches, bench_branch);
criterion_main!(benches);
