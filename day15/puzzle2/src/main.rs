use std::{
    collections::HashMap,
    io::{stdin, stdout, Read, Stdin, Write},
};

use colored::Colorize;

fn main() {
    let directions_map =
        HashMap::from([('^', (0, -1)), ('v', (0, 1)), ('<', (-1, 0)), ('>', (1, 0))]);
    let input = utils::read_file_into_list("day15/puzzle2/input");

    let mut warehouse = read_warehouse(&input);

    let directions = read_directions(&input);

    let mut robot = find_robot(&warehouse);

    print_warehouse(&warehouse);
    for (i, dir_input) in directions.iter().enumerate() {
        let dir = *directions_map.get(dir_input).unwrap();

        if *dir_input == '>' || *dir_input == '<' {
            simple_move(&mut robot, &mut warehouse, dir);
        } else {
            complex_move(&mut robot, &mut warehouse, dir);
        }
        // println!("{i}, {dir_input}");
        // print_warehouse(&warehouse);
        // println!();

        // let mut s = String::new();
        // stdin().read_line(&mut s);
        print_warehouse(&warehouse);
    }

    // print_warehouse(&warehouse);
    println!();

    let mut sum = 0;
    for y in 0..warehouse.len() {
        for x in 0..warehouse[y].len() {
            if warehouse[y][x] == '[' {
                sum += y * 100 + x;
            }
        }
    }

    println!("{sum}");
}

fn complex_move(robot: &mut (i32, i32), warehouse: &mut Vec<Vec<char>>, dir: (i32, i32)) {
    let mut current_pos = (robot.0, robot.1);
    let mut move_items: Vec<Vec<(i32, i32)>> = vec![vec![current_pos]];
    let mut cant_move = false;

    loop {
        let mut move_items_in_row = Vec::new();
        let mut is_above_free = true;

        for (i, &x) in move_items[move_items.len() - 1].iter().enumerate() {
            let above = (x.0 + dir.0, x.1 + dir.1);
            if above.0 < 0
                || above.1 < 0
                || above.0 >= warehouse[above.1 as usize].len() as i32
                || above.1 >= warehouse.len() as i32
                || warehouse[above.1 as usize][above.0 as usize] == '#'
            {
                cant_move = true;
                break;
            } else if warehouse[above.1 as usize][above.0 as usize] != '.' {
                if !move_items_in_row.contains(&above) {
                    move_items_in_row.push(above);
                }

                let warehouse_above = warehouse[above.1 as usize][above.0 as usize];
                if warehouse_above == '[' {
                    if !move_items_in_row.contains(&(above.0 + 1, above.1)) {
                        move_items_in_row.push((above.0 + 1, above.1));
                    }
                    is_above_free = false;
                } else if warehouse_above == ']' {
                    if !move_items_in_row.contains(&(above.0 - 1, above.1)) {
                        move_items_in_row.push((above.0 - 1, above.1));
                    }
                    is_above_free = false;
                }
            }
        }
        move_items.push(move_items_in_row);

        if cant_move || is_above_free {
            break;
        }
    }

    // Kann nicht verschoben werden
    if cant_move {
        return;
    }

    for row in (0..move_items.len()).rev() {
        for &mov in move_items[row].iter() {
            let new_pos = (mov.0 + dir.0, mov.1 + dir.1);
            warehouse[new_pos.1 as usize][new_pos.0 as usize] =
                warehouse[mov.1 as usize][mov.0 as usize];
            warehouse[mov.1 as usize][mov.0 as usize] = '.';
            if mov.0 == robot.0 && mov.1 == robot.1 {
                *robot = new_pos;
                break;
            }
        }
    }
}

fn simple_move(robot: &mut (i32, i32), warehouse: &mut Vec<Vec<char>>, dir: (i32, i32)) {
    let mut current_pos = (robot.0, robot.1);
    let mut cant_move = false;
    loop {
        current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
        if current_pos.0 < 0
            || current_pos.1 < 0
            || current_pos.0 >= warehouse[current_pos.1 as usize].len() as i32
            || current_pos.1 >= warehouse.len() as i32
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
        return;
    }

    loop {
        let prev_pos = (current_pos.0 - dir.0, current_pos.1 - dir.1);
        warehouse[current_pos.1 as usize][current_pos.0 as usize] =
            warehouse[prev_pos.1 as usize][prev_pos.0 as usize];
        warehouse[prev_pos.1 as usize][prev_pos.0 as usize] = '.';
        if prev_pos.0 == robot.0 && prev_pos.1 == robot.1 {
            *robot = current_pos;
            break;
        }
        current_pos = prev_pos;
    }
}

fn print_warehouse(warehouse: &Vec<Vec<char>>) {
    clearscreen::clear().unwrap();
    let mut newline = String::new();
    for y in 0..warehouse.len() {
        for x in 0..warehouse[y].len() {
            if warehouse[y][x] == '#' {
                newline.push_str(String::from(warehouse[y][x]).red().to_string().as_str());
            } else if warehouse[y][x] == '[' || warehouse[y][x] == ']' {
                newline.push_str(String::from(warehouse[y][x]).green().to_string().as_str());
            } else if warehouse[y][x] == '@' {
                newline.push_str(String::from(warehouse[y][x]).blue().to_string().as_str());
            } else {
                newline.push_str(String::from(warehouse[y][x]).as_str());
            }
        }
        newline.push('\n');
        stdout().flush().unwrap();
    }
    print!("{newline}");
}

fn read_warehouse(input: &Vec<String>) -> Vec<Vec<char>> {
    return input
        .iter()
        .take_while(|&x| !x.is_empty())
        .map(|line| {
            line.as_bytes()
                .into_iter()
                .flat_map(|&xu8| {
                    let x = xu8 as char;
                    if x == '.' || x == '#' {
                        [x as char, x as char]
                    } else if x == '@' {
                        ['@', '.']
                    } else {
                        ['[', ']']
                    }
                })
                .collect()
        })
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
