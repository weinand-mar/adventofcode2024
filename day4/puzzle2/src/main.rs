fn main() {
    let input = utils::read_file_into_list("day4/puzzle2/list");

    let x = input[0].len();
    let y = input.len();

    let mut sum = 0;
    for yi in 0..y - 2 {
        for xi in 0..x - 2 {
            let mut dig1 = String::new();
            dig1.push(input[yi].as_bytes()[xi] as char);
            dig1.push(input[yi+1].as_bytes()[xi+1] as char);
            dig1.push(input[yi+2].as_bytes()[xi+2] as char);

            let mut dig2 = String::new();
            dig2.push(input[yi].as_bytes()[xi+2] as char);
            dig2.push(input[yi+1].as_bytes()[xi+1] as char);
            dig2.push(input[yi+2].as_bytes()[xi] as char);

            if (dig1 == "MAS" || dig1 == "SAM") && (dig2 == "MAS" || dig2 == "SAM") {
                sum += 1;
            }
        }
    }
    println!("{sum}");
}
