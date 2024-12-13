use std::{ops::Mul, str::FromStr};

use num_bigint::BigInt;
use regex::Regex;
fn main() {
    let input = utils::read_file_into_list("day13/puzzle1/input");
    // let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let mut iter = input.into_iter();

    let re = Regex::new(r"Button [AB]: X\+(\d*), Y\+(\d*)").unwrap();
    let re2 = Regex::new(r"Prize: X=(\d*), Y=(\d*)").unwrap();
    let mut token = 0;
    loop {
        let next = iter.next();
        if next.is_none() {
            break;
        }

        let l1 = next.unwrap();
        let l2 = iter.next().unwrap();
        let l3 = iter.next().unwrap();
        iter.next();

        let l1_captures = re.captures(&l1).unwrap();
        let l2_captures = re.captures(&l2).unwrap();
        let l3_captures = re2.captures(&l3).unwrap();

        let ax = l1_captures[1].parse::<i128>().unwrap();
        let ay = l1_captures[2].parse::<i128>().unwrap();
        let bx = l2_captures[1].parse::<i128>().unwrap();
        let by = l2_captures[2].parse::<i128>().unwrap();
        let x = l3_captures[1].parse::<i128>().unwrap() + 10000000000000;
        let y = l3_captures[2].parse::<i128>().unwrap() + 10000000000000;

        let b1 = y * ax - ay * x;
        let b2 = by * ax - bx * ay;
        let b = b1 / b2;

        // Überprüfen ob die Division ganzzahlig war
        if b * b2 != b1 {
            continue;
        }
        let a = (x - bx * b) / ax;

        // Überprüfen ob die Division ganzzahlig war
        if a * ax != x - bx * b {
            continue;
        }

        if a < 0 || b < 0 {
            continue;
        }

        let t = a * 3 + b;

        token += t;
        println!("Button A: X+{}, Y={}", ax, ay);
        println!("Button B: X+{}, Y={}", bx, by);
        println!("Prize: X={}, y={}", x, y);
        println!("a={a}, y={b}");
        println!();
    }
    println!("{token}");
}
