use std::time::Instant;
fn main() {
    let length: usize = 14;
    let input: i64 = 12387348927323;
    let x = (10 as i64).pow((length / 2) as u32);

    let now0 = Instant::now();
    for _ in 1..10000000 {
    }
    let elapsed0 = now0.elapsed();

    let now1 = Instant::now();
    let mut sum = 0;
    for _ in 1..10000000 {
        let first = input / x;
        let second = input % x;
        sum += first + second;
    }
    println!("{sum}");
    let elapsed1 = now1.elapsed();

    let now2 = Instant::now();
    sum = 0;
    for _ in 1..10000000 {
        let first2 = input.to_string()[..length / 2].parse::<i64>().unwrap();
        let second2 = input.to_string()[length / 2..].parse::<i64>().unwrap();
        sum += first2 + second2;
    }
    println!("{sum}");
    let elapsed2 = now2.elapsed();

    println!("Elapsed0: {:.2?}", elapsed0);
    println!("Elapsed1: {:.2?}", elapsed1);
    println!("Elapsed2: {:.2?}", elapsed2);
}
