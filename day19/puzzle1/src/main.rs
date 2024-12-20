use std::collections::HashMap;


fn main() {
    let input = utils::read_file_into_list("day19/puzzle1/input");
    let towels: Vec<String> = input[0].split(", ").map(|x| String::from(x)).collect();
    let mut c = HashMap::new();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for pattern in input.iter().skip(2) {
        let cur_sum = find_all_combinations(pattern, &towels, &mut c);
        sum2 += cur_sum;
        if cur_sum > 0 {
            sum1 += 1;
        }
    }
    println!("Part 1 {}, Part 2 {}", sum1, sum2);
}

fn find_all_combinations(input: &String, towels: &Vec<String>, c: &mut HashMap<String, usize>) -> usize {
    let mut sum = 0;
    if let Some(&cached) = c.get(input) {
        return cached;
    }
    for t in towels {
        if t.len() <= input.len() {
            if &input[0..t.len()] == t.as_str() {
                if t.len() == input.len() {
                    sum += 1;
                } else {
                    let sub_input = input[t.len()..].to_string();
                    let combinations = find_all_combinations(&sub_input, towels, c);
                    c.insert(sub_input, combinations);
                    sum += combinations
                }
            }
        }
    }
    return sum;
}
