use regex::Regex;
fn main() {
    let input = utils::read_file_into_list("day13/puzzle1/input");
    // let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let mut iter = input.into_iter();

    let re = Regex::new(r"Button [AB]: X\+(\d*), Y\+(\d*)").unwrap();
    let re2 = Regex::new(r"Prize: X=(\d*), Y=(\d*)").unwrap();
    let mut token = 0;
    let mut token_part2 = 0;
    loop {
        let next = iter.next();
        if next.is_none() {
            break;
        }

        let mut is_valid = true;
        let mut is_valid_part2 = true;

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
        let x = l3_captures[1].parse::<i128>().unwrap();
        let y = l3_captures[2].parse::<i128>().unwrap();

        let x_part2 = l3_captures[1].parse::<i128>().unwrap() + 10000000000000;
        let y_part2 = l3_captures[2].parse::<i128>().unwrap() + 10000000000000;

        let b1 = y * ax - ay * x;
        let b2 = by * ax - bx * ay;
        let b = b1 / b2;

        let b1_part2 = y_part2 * ax - ay * x_part2;
        let b_part2 = b1_part2 / b2;

        // Überprüfen ob die Division ganzzahlig war
        if b * b2 != b1 {
            is_valid = false;
        }
        if b_part2 * b2 != b1_part2 {
            is_valid_part2 = false;
        }

        let a = (x - bx * b) / ax;
        let a_part2 = (x_part2 - bx * b_part2) / ax;

        // Überprüfen ob die Division ganzzahlig war
        if a * ax != x - bx * b {
            is_valid = false;
        }
        if a_part2 * ax != x_part2 - bx * b_part2 {
            is_valid_part2 = false;
        }

        if a < 0 || b < 0 {
            is_valid = false;
        }
        if a_part2 < 0 || b_part2 < 0 {
            is_valid_part2 = false;
        }

        if a > 100 || b > 100 {
            is_valid = false;
        }

        if is_valid {
            let t = a * 3 + b;
            token += t;
        }
        if is_valid_part2 {
            let t = a_part2 * 3 + b_part2;
            token_part2 += t;
        }
    }
    println!("Part1 = {token}");
    println!("Part2 = {token_part2}");
}
