fn main() {
    let input = std::fs::read_to_string("day9/puzzle2/input").unwrap();

    let mut result: Vec<String> = Vec::new();

    let mut index = 0;
    let mut block = true;
    for i in input.as_bytes() {
        let c = match block {
            true => index.to_string(),
            false => ".".to_owned(),
        };

        for _ in 0..(*i - '0' as u8) {
            result.push(c.clone());
        }

        block = !block;
        if !block {
            index += 1;
        }
    }

    let mut end = result.len() - 1;
    let mut end_start: usize;

    while end != 0 {
        while end != 0 && result[end] == "." {
            end -= 1;
        }
        end_start = end;
        while end_start != 0 && result[end_start] == result[end] {
            end_start -= 1;
        }

        let mut start = 0;
        let mut start_end = 0;
        while start_end - start < end - end_start && start < end {
            start = start_end;
            while start < result.len() && result[start] != "." {
                start += 1;
            }
            start_end = start;
            while start_end < result.len() && result[start_end] == "." {
                start_end += 1;
            }
        }

        if start < end {
            if start_end - start >= end - end_start {
                for i in 0..(end - end_start) {
                    let temp = result[start + i].clone();
                    result[start + i] = result[end - i].clone();
                    result[end - i] = temp;
                }
            } else {
                end = end_start;
            }
        } else {
            end = end_start;
        }
    }

    let mut sum: u64 = 0;
    let mut mult = 0;
    for i in 0..result.len() {
        if result[i] != "." {
            let number = result[i].parse::<u64>().unwrap();
            sum += number * mult;
        }
        mult += 1;
    }

    println!("{sum}");
}
