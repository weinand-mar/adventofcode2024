use std::collections::HashMap;

fn main() {
    let lines = utils::read_file_into_list("day5/puzzle1/input");

    let position = lines.iter().position(|x| x == "").unwrap();

    let rules_raw = &lines[0..position];
    let rules: Vec<(i32, i32)> = rules_raw
        .iter()
        .map(|x| x.split("|").collect())
        .map(|x: Vec<&str>| (x[0].parse::<i32>().unwrap(), x[1].parse::<i32>().unwrap()))
        .collect();

    let mut rule_first: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut rule_second: HashMap<i32, Vec<i32>> = HashMap::new();

    for r in rules {
        if rule_first.contains_key(&r.0) {
            let get = rule_first.get_mut(&r.0).unwrap();
            get.push(r.1);
        } else {
            rule_first.insert(r.0, vec![r.1]);
        }

        if rule_second.contains_key(&r.1) {
            let get = rule_second.get_mut(&r.1).unwrap();
            get.push(r.0);
        } else {
            rule_second.insert(r.1, vec![r.0]);
        }
    }

    let inputs: Vec<Vec<i32>> = (&lines[position + 1..])
        .iter()
        .map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap()).collect())
        .collect();

    let mut sum = 0;
    let mut invalid_inputs: Vec<Vec<i32>> = Vec::new();
    let len_inputs = inputs.len();
    for input in inputs {
        if is_input_valid(&input, &rule_first, &rule_second) {
            sum += input[input.len() / 2];
        }
        else {
            invalid_inputs.push(input);
        }
    }
    println!("{sum}");

    println!("{}, {}", len_inputs, invalid_inputs.len());

    let mut sum_fixed = 0;
    for input in invalid_inputs {
        let fixed = fix_input(&input, &rule_first, &rule_second);
        sum_fixed += fixed[fixed.len() / 2];
        println!("{:?}, {:?}", input, fixed);
    }
    println!("{sum_fixed}");


}

fn fix_input(input: &Vec<i32>, rule_first: &HashMap<i32, Vec<i32>>, rule_second: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut fix_input = input.clone();
    let mut need_fixes = true;
    while need_fixes {
        let mut no_fixes_this_iteration = true;
        for i in 0..fix_input.len() {
            let r = find_error_index(i, &fix_input, &rule_first, &rule_second);
            if r >= 0 {
                let temp = fix_input[i];
                fix_input[i] = fix_input[r as usize];
                fix_input[r as usize] = temp;
                no_fixes_this_iteration = false;
                break;
            }
        }
        need_fixes = !no_fixes_this_iteration;
    }
    return fix_input;
}


fn is_input_valid(input: &Vec<i32>, rule_first: &HashMap<i32, Vec<i32>>, rule_second: &HashMap<i32, Vec<i32>>) -> bool {
    for i in 0..input.len() {
        if find_error_index(i, &input, &rule_first, &rule_second) >= 0 {
            return false;
        }
    }
    return true;
}


fn find_error_index(i: usize, input: &Vec<i32>, rule_first: &HashMap<i32, Vec<i32>>, rule_second: &HashMap<i32, Vec<i32>>) -> i32 {
    let e = input[i];
    match rule_first.get(&e) {
        Some(rule) => {
            let r = is_before(&input, i, rule);
            if r >= 0 {
                return r;
            }
        }
        None => (),
    }

    match rule_second.get(&e) {
        Some(rule) => {
            let r = is_after(&input, i, rule);
            if r >= 0 {
                return r;
            }
        }
        None => (),
    }
    return -1;
}

fn is_before(input: &Vec<i32>, x: usize, rule: &[i32]) -> i32 {
    for i in x..0 {
        let e = input[i];
        if rule.contains(&e) {
            return i as i32;
        }
    }
    return -1;
}

fn is_after(input: &Vec<i32>, x: usize, rule: &[i32]) -> i32 {
    for i in (x+1)..input.len() {
        let e = input[i];
        if rule.contains(&e) {
            return i as i32;
        }
    }
    return -1;
}
