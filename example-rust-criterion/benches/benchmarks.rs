use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use example_rust_criterion::*;

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    
    for n in [10, 20, 30].iter() {
        group.bench_with_input(BenchmarkId::new("recursive", n), n, |b, n| {
            b.iter(|| fibonacci_recursive(black_box(*n)))
        });
        
        group.bench_with_input(BenchmarkId::new("iterative", n), n, |b, n| {
            b.iter(|| fibonacci_iterative(black_box(*n)))
        });
    }
    
    group.finish();
}

fn bench_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");
    
    let sizes = [100, 1000, 5000];
    
    for size in sizes.iter() {
        let data: Vec<i32> = (0..*size).rev().collect();
        
        group.bench_with_input(BenchmarkId::new("bubble_sort", size), &data, |b, data| {
            b.iter(|| bubble_sort(black_box(data.clone())))
        });
        
        group.bench_with_input(BenchmarkId::new("quick_sort", size), &data, |b, data| {
            b.iter(|| quick_sort(black_box(data.clone())))
        });
    }
    
    group.finish();
}

fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");
    
    let strings = vec!["hello", "world", "rust", "benchmark", "criterion"];
    
    group.bench_function("concatenation", |b| {
        b.iter(|| string_concatenation(black_box(&strings)))
    });
    
    group.bench_function("join_with_space", |b| {
        b.iter(|| string_join(black_box(&strings), black_box(" ")))
    });
    
    group.bench_function("join_with_comma", |b| {
        b.iter(|| string_join(black_box(&strings), black_box(", ")))
    });
    
    group.finish();
}

fn bench_hash_map(c: &mut Criterion) {
    c.bench_function("hash_map_operations", |b| {
        b.iter(|| hash_map_operations())
    });
}

fn bench_vector_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_operations");
    
    let sizes = [1000, 10000, 100000];
    
    for size in sizes.iter() {
        group.bench_with_input(BenchmarkId::new("vec_push", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    vec.push(black_box(i));
                }
                vec
            })
        });
        
        group.bench_with_input(BenchmarkId::new("vec_with_capacity", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::with_capacity(size);
                for i in 0..size {
                    vec.push(black_box(i));
                }
                vec
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_fibonacci,
    bench_sorting,
    bench_string_operations,
    bench_hash_map,
    bench_vector_operations
);
criterion_main!(benches);