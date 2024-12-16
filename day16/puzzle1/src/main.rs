use colored::Colorize;
use std::{
    collections::{HashSet, LinkedList},
    io::stdin,
    usize,
};

fn main() {
    let input = utils::read_file_into_list_of_list_of_chars("day16/puzzle1/input");
    let start = search_letter(&input, 'S');
    let end = search_letter(&input, 'E');

    let paths_to_end = dfs(&input, start, end);
    let (costs, path) = find_sheapest_path(&input, &paths_to_end);

    print_maze(&input, &path);
    println!();
    println!("Costs: {costs}");
}

fn find_sheapest_path(
    input: &Vec<Vec<char>>,
    paths: &Vec<Vec<(usize, usize)>>,
) -> (usize, Vec<(usize, usize)>) {
    let mut cheapest_path: Vec<(usize, usize)> = Vec::new();
    let mut cheapest_costs = usize::MAX;
    for path in paths {
        // print_maze(input, path);
        let mut dir = (1, 0);
        let mut costs = path.len() - 1;
        for i in 1..path.len() {
            let new_dir = (
                path[i - 1].0 as i32 - path[i].0 as i32,
                path[i - 1].1 as i32 - path[i].1 as i32,
            );
            if dir != new_dir {
                costs += 1000;
            }
            dir = new_dir;
        }
        if costs < cheapest_costs {
            cheapest_costs = costs;
            cheapest_path = path.clone();
        }
    }
    return (cheapest_costs, cheapest_path);
}

fn dfs(
    input: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<(usize, usize)>> {
    let mut stack = LinkedList::new();
    stack.push_back(vec![start]);
    let mut path_to_end: Vec<Vec<(usize, usize)>> = Vec::new();

    while !stack.is_empty() {
        let pop = stack.pop_front().unwrap();
        let last = pop[pop.len() - 1];
        let (x, y) = last;

        // print_maze(input, &pop);

        if last == end {
            print_maze(input, &pop);
            path_to_end.push(pop);
            continue;
        }
        if x > 0 && input[y][x - 1] != '#' && !pop.contains(&(x - 1, y)) {
            let mut vec = pop.clone();
            vec.push((x - 1, y));
            stack.push_back(vec);
        }
        if x < input.len() - 1 && input[y][x + 1] != '#' && !pop.contains(&(x + 1, y)) {
            let mut vec = pop.clone();
            vec.push((x + 1, y));
            stack.push_back(vec);
        }
        if y > 0 && input[y - 1][x] != '#' && !pop.contains(&(x, y - 1)) {
            let mut vec = pop.clone();
            vec.push((x, y - 1));
            stack.push_back(vec);
        }
        if y < input.len() - 1 && input[y + 1][x] != '#' && !pop.contains(&(x, y + 1)) {
            let mut vec = pop.clone();
            vec.push((x, y + 1));
            stack.push_back(vec);
        }
    }

    return path_to_end;
}

fn print_maze(input: &Vec<Vec<char>>, path: &Vec<(usize, usize)>) {
    clearscreen::clear().unwrap();
    let mut s = String::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if path.contains(&(x, y)) {
                s.push_str(String::from('*').red().to_string().as_str());
            } else {
                s.push(input[y][x]);
            }
        }
        s.push('\n');
    }
    println!("{s}");
}

fn print_maze_2(
    input: &Vec<Vec<char>>,
    path: &Vec<(usize, usize)>,
    visited: &HashSet<(usize, usize)>,
) {
    clearscreen::clear().unwrap();
    let mut s = String::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if path.contains(&(x, y)) {
                if visited.contains(&(x, y)) {
                    s.push_str(String::from('*').green().to_string().as_str());
                } else {
                    s.push_str(String::from('*').red().to_string().as_str());
                }
            } else {
                if visited.contains(&(x, y)) {
                    s.push_str(String::from(input[y][x]).green().to_string().as_str());
                } else {
                    s.push(input[y][x]);
                }
            }
        }
        s.push('\n');
    }
    println!("{s}");
}

fn search_letter(input: &Vec<Vec<char>>, letter: char) -> (usize, usize) {
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == letter {
                return (x, y);
            }
        }
    }
    return (0, 0);
}
