use std::collections::HashMap;

fn main() {
    let directions_map =
        HashMap::from([('^', (0, -1)), ('v', (0, 1)), ('<', (-1, 0)), ('>', (1, 0))]);
    let input = utils::read_file_into_list("day15/puzzle1/input");

    let mut warehouse = read_warehouse(&input);

    let directions = read_directions(&input);

    let mut robot = find_robot(&warehouse);

    println!("{:?}", directions);
    print_warehouse(&warehouse);
    for (i, dir_input) in directions.iter().enumerate() {
        let dir = *directions_map.get(dir_input).unwrap();

        let mut current_pos = (robot.0, robot.1);
        let mut cant_move = false;
        loop {
            current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
            if current_pos.0 < 0
                || current_pos.1 < 0
                || current_pos.0 >= warehouse.len() as i32
                || current_pos.1 >= warehouse[current_pos.0 as usize].len() as i32
                || warehouse[current_pos.1 as usize][current_pos.0 as usize] == '#'
            {
                cant_move = true;
                break;
            }
            if warehouse[current_pos.1 as usize][current_pos.0 as usize] == '.' {
                break;
            }
        }

        // Kann nicht verschoben werden
        if cant_move {
            // println!("{i}, {dir_input}");
            // print_warehouse(&warehouse);
            // println!();
            continue;
        }

        loop {
            let prev_pos = (current_pos.0 - dir.0, current_pos.1 - dir.1);
            warehouse[current_pos.1 as usize][current_pos.0 as usize] =
                warehouse[prev_pos.1 as usize][prev_pos.0 as usize];
            warehouse[prev_pos.1 as usize][prev_pos.0 as usize] = '.';
            if prev_pos.0 == robot.0 && prev_pos.1 == robot.1 {
                robot = current_pos;
                break;
            }
            current_pos = prev_pos;
        }
        // println!("{i}, {dir_input}");
        // print_warehouse(&warehouse);
        // println!();
    }

    print_warehouse(&warehouse);
    println!();

    let mut sum = 0;
    for y in 0..warehouse.len() {
        for x in 0..warehouse[y].len() {
            if warehouse[y][x] == 'O' {
                sum += y * 100 + x;
            }
        }
    }

    println!("{sum}");
}

fn print_warehouse(warehouse: &Vec<Vec<char>>) {
    for y in 0..warehouse.len() {
        for x in 0..warehouse[y].len() {
            print!("{}", warehouse[y][x]);
        }
        println!();
    }
}

fn read_warehouse(input: &Vec<String>) -> Vec<Vec<char>> {
    return input
        .iter()
        .take_while(|&x| !x.is_empty())
        .map(|line| line.as_bytes().into_iter().map(|&x| x as char).collect())
        .collect();
}

fn find_robot(warehouse: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..warehouse.len() {
        for x in 0..warehouse[y].len() {
            if warehouse[y][x] == '@' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!();
}

fn read_directions(input: &Vec<String>) -> Vec<char> {
    return input
        .iter()
        .skip_while(|&x| !x.is_empty())
        .skip(1)
        .flat_map(|line| {
            line.as_bytes()
                .into_iter()
                .map(|&x| x as char)
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>();
}
