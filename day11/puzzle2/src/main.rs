use std::collections::HashMap;
use std::thread::{self, JoinHandle};
fn main() {
    let input: Vec<i64> = vec![5, 127, 680267, 39260, 0, 26, 3553, 5851995];
    let mut threads: Vec<JoinHandle<i64>> = Vec::new();

    for i in 0..input.len() {
        let number = input[i];
        let handle = thread::spawn(move || handle(0, number, &mut HashMap::new()));
        threads.push(handle);
    }

    let mut sum: i64 = 0;
    for t in threads {
        let join = t.join().unwrap();
        sum += join;
    }
    println!("{sum}");
}

fn len_of_number(i: i64) -> usize {
    return i.to_string().len();
}

fn handle(depth: i32, number: i64, cache: &mut HashMap<(i32, i64), i64>) -> i64 {
    if depth == 75 {
        return 1;
    }
    if cache.contains_key(&(depth, number)) {
        return *cache.get(&(depth, number)).unwrap();
    }
    let length = len_of_number(number);
    let sum: i64 = if number == 0 {
        handle(depth + 1, 1, cache)
    } else if length % 2 == 0 {
        let x = (10 as i64).pow((length / 2) as u32);
        let first = number / x;
        let second = number % x;
        handle(depth + 1, first, cache) + handle(depth + 1, second, cache)
    } else {
        handle(depth + 1, number * 2024, cache)
    };
    cache.insert((depth, number), sum);
    return sum;
}
