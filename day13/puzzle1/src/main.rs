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

        let sub_1 = y * ax;
        let sub_2 = ay * x;
        let sub_3 = by * ax;
        let sub_4 = bx * ay;

        // Div Zero überprung und Überprüfung ob Ergebnis der Division negativ ist
        if sub_3 == sub_4 || sub_1 < sub_2 && sub_3 > sub_4 || sub_1 > sub_2 && sub_3 < sub_4 {
            continue;
        }

        let b1 = sub_1 - sub_2;
        let b2 = sub_3 - sub_4;
        let b = b1 / b2;

        // Überprüfen ob die Division ganzzahlig war
        if b * b2 != b1 {
            continue;
        }

        // ax kann nie negativ oder 0 sein, daher hier nur überprüfen ob Nenner negativ ist
        if x < bx * b {
            continue;
        }
        let a = (x - bx * b) / ax;

        // Überprüfen ob die Division ganzzahlig war
        if a * ax != (x - bx * b) {
            continue;
        }
        let t = a + b * 3;

        token += t;
        println!("Button A: X+{}, Y={}", ax, ay);
        println!("Button B: X+{}, Y={}", bx, by);
        println!("Prize: X={}, y={}", x, y);
        println!("a={a}, y={b}");
        println!();
    }
    println!("{token}");
    // 2906507091332038656
    // 2906507091332038656
    // 96134449629749
    // 436537413428004568
    // 2858055499541968099
    // 1851851851852001
}
