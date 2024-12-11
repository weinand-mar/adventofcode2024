fn main() {
    let mut input:Vec<i64> = Vec::with_capacity(10000000);
    let mut foo:Vec<i64> = vec![5, 127, 680267, 39260, 0, 26, 3553, 5851995];
    // let mut foo:Vec<i64> = vec![5];
    input.append(&mut foo);
    // let mut input:Vec<i64> = vec![125, 17];
    for p in 0..75 {
        let mut i = 0;
        let org_len = input.len();
        let mut last_len = org_len;
        while i < org_len {
            let length = len_of_number(input[i]);
            if input[i] == 0 {
                input[i] = 1;
            } else if length % 2 == 0 {
                let x = (10 as i64).pow((length / 2) as u32) ;
                let first = input[i] / x;
                let second = input[i] % x;
                // input.splice(i..i+1, vec![first, second]);
                // i += 1;
                // let first = input[i].to_string()[..length/2].parse::<i64>().unwrap();
                // let second = input[i].to_string()[length/2..].parse::<i64>().unwrap();
                input[i] = first;
                input.push(second);
            }
            else {
                input[i] *= 2024;
            }
            i += 1;
        }
        println!("{p}, {}, {}, {}", input.len(), input.len() - last_len, input.len() as f64 / last_len as f64);
        last_len = input.len();
        // println!("{}", input.len());
    }
    println!("{}", input.len());
}

fn len_of_number(i: i64) -> usize {
    return i.to_string().len();
}
