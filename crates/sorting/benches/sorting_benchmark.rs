use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rand::{distributions::Uniform, Rng};

use sorting::algorithm::SortingAlgorithm;

#[macro_export]
macro_rules! generate_input {
    ($size:expr) => {{
        let mut rng = rand::thread_rng();
        let range = Uniform::new(-500, 500);
        (0..$size).map(|_| rng.sample(&range)).collect::<Vec<i64>>()
    }};
}

fn sorting_bechmark(c: &mut Criterion) {
    let mut sorting_group = c.benchmark_group("Sorting");
    for input_items in [
        generate_input!(50),
        generate_input!(100),
        generate_input!(500),
        generate_input!(1_000),
        generate_input!(10_000),
        generate_input!(20_000),
        generate_input!(50_000),
        generate_input!(70_000),
        generate_input!(100_000),
    ] {
        sorting_group.throughput(Throughput::Elements(input_items.len() as u64));
        sorting_group.bench_with_input("Selection Sort", &input_items, |b, input| {
            b.iter(|| SortingAlgorithm::Selection.sort_mut(black_box(&mut input.to_owned())))
        });
        sorting_group.bench_with_input("Insertion Sort", &input_items, |b, input| {
            b.iter(|| SortingAlgorithm::Insertion.sort_mut(black_box(&mut input.to_owned())));
        });
        sorting_group.bench_with_input("Merge Sort", &input_items, |b, input| {
            b.iter(|| SortingAlgorithm::Merge.sort_mut(black_box(&mut input.to_owned())));
        });
    }
}

criterion_group!(benches, sorting_bechmark);
criterion_main!(benches);
