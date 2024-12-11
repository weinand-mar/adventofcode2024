use std::collections::HashMap;

fn main() {
    let raw_input = utils::read_file_into_list_of_list_of_chars("day6/puzzle2/input");

    let mut sum = 0;
    for x in 0..raw_input.len() {
        for y in 0..raw_input[x].len() {
            let mut input = raw_input.clone();
            if input[x][y] != '#' && input[x][y] != '^' && input[x][y] != 'v' && input[x][y] != '<' && input[x][y] != '>' {
                input[x][y] = 'O';
                if loops(&mut input) {
                    sum += 1;
                }
            }
        }
    }
    println!("{sum}");
}

fn loops(input: &mut Vec<Vec<char>>) -> bool {
    let mut pos = search_guard(&input);
    let mut count_rotate = 0;
    let mut visited_map: HashMap<(usize,usize), Vec<char>> = HashMap::new();
    loop {
        match can_move(pos, &input) {
            Some(can_move_result) => {
                if can_move_result {
                    count_rotate = 0;
                    let move_result = move_guard(pos, input);
                    pos = move_result.new_pos;
                } else {
                    let stuck_in_loop = is_stuck_in_loop_and_update_map(&mut visited_map, pos, input);
                    if stuck_in_loop {
                        return true;
                    }
                    rotate(pos, input);
                    count_rotate += 1;
                    if count_rotate == 2 {
                        println!("doppelt rotiert");
                    }
                }
            }
            None => return false,
        }
    }
}

fn is_stuck_in_loop_and_update_map(visited_map: &mut HashMap<(usize,usize), Vec<char>>, pos: (usize, usize), input: &Vec<Vec<char>>) -> bool {
    match visited_map.get_mut(&pos) {
        Some(visited_directions) => {
            if visited_directions.contains(&input[pos.0][pos.1]) {
                return true;
            }
            visited_directions.push(input[pos.0][pos.1]);
            return false;
        },
        None => {
            visited_map.insert(pos, vec![input[pos.0][pos.1]]);
            return false;
        }
    };
}

struct MoveResult {
    new_pos:(usize, usize),
}

fn move_guard(pos: (usize, usize), input: &mut Vec<Vec<char>>) -> MoveResult {
    let new_pos = match input[pos.0][pos.1] {
        '^' => (pos.0 - 1, pos.1),
        'v' => (pos.0 + 1, pos.1),
        '<' => (pos.0, pos.1 - 1),
        '>' => (pos.0, pos.1 + 1),
        _ => panic!("Invalid direction"),
    };

    input[new_pos.0][new_pos.1] = input[pos.0][pos.1];
    input[pos.0][pos.1] = 'X';
    return MoveResult {
        new_pos: new_pos,
    };
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
                Some(input[pos.0 - 1][pos.1] != '#' && input[pos.0 - 1][pos.1] != 'O')
            }
        }
        'v' => {
            return if pos.0 + 1 == input.len() {
                None
            } else {
                Some(input[pos.0 + 1][pos.1] != '#' && input[pos.0 + 1][pos.1] != 'O')
            }
        }
        '<' => {
            return if pos.1 == 0 {
                None
            } else {
                Some(input[pos.0][pos.1 - 1] != '#' && input[pos.0][pos.1 - 1] != 'O')
            }
        }
        '>' => {
            return if pos.1 + 1 == input[0].len() {
                None
            } else {
                Some(input[pos.0][pos.1 + 1] != '#' && input[pos.0][pos.1 + 1] != 'O')
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
