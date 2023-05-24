use criterion::{criterion_group, criterion_main, Bencher, Criterion, Throughput};
use heck::AsSnakeCase;
use rand::{distributions::Uniform, Rng};

use sorting::algorithm::SortingAlgorithm;

macro_rules! generate_input {
    ($size:expr) => {{
        let mut rng = rand::thread_rng();
        let range = Uniform::new(-($size >> 1), ($size >> 1));
        (0..$size).map(|_| rng.sample(&range)).collect::<Vec<i64>>()
    }};
}

macro_rules! create_bench_function {
    ($x:ident) => {
        |b: &mut Bencher, input: &Vec<i64>| {
            b.iter(|| SortingAlgorithm::$x.sort_mut(&mut input.to_owned()))
        }
    };
}

macro_rules! create_bench {
    ($group: expr, $sizes: expr, $($fn_name: ident), *) => {
        for size in $sizes {
            let input_items = generate_input!(size);
            $group.throughput(Throughput::Elements(input_items.len() as u64));
            $($group.bench_with_input(format!("{}_sort", AsSnakeCase(stringify!($fn_name))), &input_items, create_bench_function!($fn_name));)*
        }
    };
}

fn sorting_bechmark(c: &mut Criterion) {
    let sizes = vec![
        50, 100, 500, 1_000, 2_000, 5_000, 10_000, 20_000, 50_000, 70_000, 100_000, 500_000,
        1_000_000,
    ];

    let mut sorting_group = c.benchmark_group("sorting_bechmark");
    create_bench!(sorting_group, sizes, Selection, Insertion, Merge);
}

criterion_group!(benches, sorting_bechmark);
criterion_main!(benches);
