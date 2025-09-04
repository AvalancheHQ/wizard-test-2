use divan::black_box;
use example_rust_divan::*;

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 20, 30])]
fn fibonacci_recursive_bench(bencher: divan::Bencher, n: u64) {
    bencher.bench(|| fibonacci_recursive(black_box(n)));
}

#[divan::bench(args = [10, 20, 30])]
fn fibonacci_iterative_bench(bencher: divan::Bencher, n: u64) {
    bencher.bench(|| fibonacci_iterative(black_box(n)));
}

#[divan::bench(args = [100, 1000, 5000])]
fn bubble_sort_bench(bencher: divan::Bencher, size: usize) {
    let data: Vec<i32> = (0..size as i32).rev().collect();
    
    bencher
        .with_inputs(|| data.clone())
        .bench_values(|data| bubble_sort(black_box(data)));
}

#[divan::bench(args = [100, 1000, 5000])]
fn quick_sort_bench(bencher: divan::Bencher, size: usize) {
    let data: Vec<i32> = (0..size as i32).rev().collect();
    
    bencher
        .with_inputs(|| data.clone())
        .bench_values(|data| quick_sort(black_box(data)));
}

#[divan::bench]
fn string_concatenation_bench(bencher: divan::Bencher) {
    let strings = vec!["hello", "world", "rust", "benchmark", "divan"];
    
    bencher.bench(|| string_concatenation(black_box(&strings)));
}

#[divan::bench]
fn string_join_space_bench(bencher: divan::Bencher) {
    let strings = vec!["hello", "world", "rust", "benchmark", "divan"];
    
    bencher.bench(|| string_join(black_box(&strings), black_box(" ")));
}

#[divan::bench]
fn string_join_comma_bench(bencher: divan::Bencher) {
    let strings = vec!["hello", "world", "rust", "benchmark", "divan"];
    
    bencher.bench(|| string_join(black_box(&strings), black_box(", ")));
}

#[divan::bench]
fn hash_map_operations_bench(bencher: divan::Bencher) {
    bencher.bench(|| hash_map_operations());
}

#[divan::bench(args = [1000, 10000, 100000])]
fn vec_push_bench(bencher: divan::Bencher, size: usize) {
    bencher.bench(|| {
        let mut vec = Vec::new();
        for i in 0..size {
            vec.push(black_box(i));
        }
        vec
    });
}

#[divan::bench(args = [1000, 10000, 100000])]
fn vec_with_capacity_bench(bencher: divan::Bencher, size: usize) {
    bencher.bench(|| {
        let mut vec = Vec::with_capacity(size);
        for i in 0..size {
            vec.push(black_box(i));
        }
        vec
    });
}