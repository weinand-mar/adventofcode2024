fn main() {
    let mut input = utils::read_file_into_list_of_list_of_chars("day6/puzzle1/input");

    println!("{:?}", search_guard(&input));

    let mut pos = search_guard(&input);

    let mut out_of_bounds = false;
    while !out_of_bounds {
        match can_move(pos, &input) {
            Some(can_move_result) => {
                if can_move_result {
                    pos = move_guard(pos, &mut input);
                } else {
                    rotate(pos, &mut input);
                }
            }
            None => out_of_bounds = true,
        }
    }
    
    input[pos.0][pos.1] = 'X';

    let result: usize = input.iter()
    .map(|x| x.iter().filter(|&&y| y == 'X').count())
    .sum();

    for x in input {
        println!("{:?}", x);
    }

    println!("{result}");
}

fn move_guard(pos: (usize, usize), input: &mut Vec<Vec<char>>) -> (usize, usize) {
    let new_pos = match input[pos.0][pos.1] {
        '^' => (pos.0 - 1, pos.1),
        'v' => (pos.0 + 1, pos.1),
        '<' => (pos.0, pos.1 - 1),
        '>' => (pos.0, pos.1 + 1),
        _ => panic!("Invalid direction"),
    };

    input[new_pos.0][new_pos.1] = input[pos.0][pos.1];
    input[pos.0][pos.1] = 'X';
    return new_pos;
}

fn rotate(pos: (usize, usize), input: &mut Vec<Vec<char>>) {
    match input[pos.0][pos.1] {
        '^' => input[pos.0][pos.1] = '>',
        'v' => input[pos.0][pos.1] = '<',
        '<' => input[pos.0][pos.1] = '^',
        '>' => input[pos.0][pos.1] = 'v',
        _ => panic!("Invalid direction"),
    }
}

fn can_move(pos: (usize, usize), input: &Vec<Vec<char>>) -> Option<bool> {
    match input[pos.0][pos.1] {
        '^' => {
            return if pos.0 == 0 {
                None
            } else {
                Some(input[pos.0 - 1][pos.1] != '#')
            }
        }
        'v' => {
            return if pos.0 + 1 == input.len() {
                None
            } else {
                Some(input[pos.0 + 1][pos.1] != '#')
            }
        }
        '<' => {
            return if pos.1 == 0 {
                None
            } else {
                Some(input[pos.0][pos.1 - 1] != '#')
            }
        }
        '>' => {
            return if pos.1 + 1 == input[0].len() {
                None
            } else {
                Some(input[pos.0][pos.1 + 1] != '#')
            }
        }
        _ => panic!("Invalid direction"),
    }
}

fn search_guard(input: &Vec<Vec<char>>) -> (usize, usize) {
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] == '^' || input[y][x] == 'v' || input[y][x] == '<' || input[y][x] == '>'
            {
                return (x, y);
            }
        }
    }
    return (0, 0);
}
