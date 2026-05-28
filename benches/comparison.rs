use criterion::{black_box, criterion_group, criterion_main, Criterion};
use another_game::naive::winner_naive;
use another_game::optimized::winner_optimized;

fn benchmark_game(c: &mut Criterion) {
    let heaps_small = vec![1, 2, 3, 1];
    let heaps_large = vec![5; 1000];

    let mut group = c.benchmark_group("game_solution_comparison");

    group.bench_function("naive_small", |b| {
        b.iter(|| winner_naive(black_box(&heaps_small)))
    });

    group.bench_function("optimized_small", |b| {
        b.iter(|| winner_optimized(black_box(&heaps_small)))
    });

    group.bench_function("naive_large", |b| {
        b.iter(|| winner_naive(black_box(&heaps_large)))
    });

    group.bench_function("optimized_large", |b| {
        b.iter(|| winner_optimized(black_box(&heaps_large)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_game);
criterion_main!(benches);