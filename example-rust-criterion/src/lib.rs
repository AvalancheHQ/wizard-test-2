pub fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

pub fn fibonacci_iterative(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    
    let mut prev = 0;
    let mut curr = 1;
    
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    curr
}

pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

pub fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    
    let pivot_index = partition(&mut arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    
    let mut left_sorted = quick_sort(left.to_vec());
    let mut right_sorted = quick_sort(right[1..].to_vec());
    
    left_sorted.push(arr[pivot_index]);
    left_sorted.append(&mut right_sorted);
    left_sorted
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;
    
    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, len - 1);
    i
}

pub fn string_concatenation(strings: &[&str]) -> String {
    strings.iter().cloned().collect::<String>()
}

pub fn string_join(strings: &[&str], separator: &str) -> String {
    strings.join(separator)
}

pub fn hash_map_operations() -> std::collections::HashMap<i32, String> {
    let mut map = std::collections::HashMap::new();
    
    for i in 0..1000 {
        map.insert(i, format!("value_{}", i));
    }
    
    map
}