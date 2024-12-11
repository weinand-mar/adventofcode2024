fn main() {
    let input = utils::read_file_into_list_of_list_of_numbers("day10/puzzle2/input");

    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 0 {
                let result = path_to_nine(&input, (i, j));
                sum += result
                    .iter()
                    .count();
            }
        }
    }

    println!("{sum}");
}

fn path_to_nine(input: &Vec<Vec<i32>>, start: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let mut stack = vec![vec![start]];
    let mut paths = Vec::new();
    while !stack.is_empty() {
        let pop = stack.pop().unwrap();
        let end = pop[pop.len() - 1];

        let end_number = input[end.0][end.1] + 1;

        let mut new_points = Vec::new();
        if end.0 != 0 && end_number == input[end.0 - 1][end.1] {
            new_points.push((end.0 - 1, end.1));
        }
        if end.0 != input.len() - 1 && end_number == input[end.0 + 1][end.1] {
            new_points.push((end.0 + 1, end.1));
        }
        if end.1 != 0 && end_number == input[end.0][end.1 - 1] {
            new_points.push((end.0, end.1 - 1));
        }
        if end.1 != input[end.0].len() - 1 && end_number == input[end.0][end.1 + 1] {
            new_points.push((end.0, end.1 + 1));
        }

        for new_point in new_points {
            let mut new_path = pop.clone();
            new_path.push(new_point);
            if new_path.len() == 10 {
                paths.push(new_path);
            } else {
                stack.push(new_path);
            }
        }
    }
    return paths;
}
