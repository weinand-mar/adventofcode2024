fn main() {
    let input = std::fs::read_to_string("day9/puzzle1/input").unwrap();

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

    let mut start = 0;
    let mut end = result.len() - 1;
    while start < end {
        while result[start] != "." || start == result.len() {
            start += 1;
        }

        while result[end] == "." || end == 0 {
            end -= 1;
        }

        if start < end {
            let temp = result[start].clone();
            result[start] = result[end].clone();
            result[end] = temp;
        }
    }

   let trim = &result[..result.iter().position(|x| x == ".").unwrap()];

    let mut sum: u64 = 0;
    for i in 0..trim.len() {
        let number = trim[i].parse::<u64>().unwrap();
        sum += number * i as u64;
    }

    println!("{sum}");
}
