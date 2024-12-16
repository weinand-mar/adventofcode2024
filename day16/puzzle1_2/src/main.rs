use colored::Colorize;
use std::{
    collections::{HashMap, HashSet, LinkedList},
    io::stdin,
    usize,
};

fn main() {
    let input = utils::read_file_into_list_of_list_of_chars("day16/puzzle1/input");
    let start = search_letter(&input, 'S');
    let end = search_letter(&input, 'E');

    let mut abstand: Vec<usize> = vec![usize::MAX; input.len() * input[0].len()];
    let mut vorgaenger: Vec<Option<(usize, usize)>> = vec![None; input.len() * input[0].len()];
    let mut Q: Vec<(usize, usize)> = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            Q.push((x, y));
        }
    }
    abstand[start.0 + start.1 * input.len()] = 0;

    while !Q.is_empty() {
        let (i, &u) = Q
            .iter()
            .enumerate()
            .min_by(|a, b| {
                abstand[a.1 .0 + a.1 .1 * input.len()].cmp(&abstand[b.1 .0 + b.1 .1 * input.len()])
            })
            .unwrap();
        Q.remove(i);

        let (x, y) = u;
        let mut nachbar = Vec::new();
        if x > 0 && input[y][x - 1] != '#' {
            nachbar.push((x - 1, y));
        }
        if x < input.len() - 1 && input[y][x + 1] != '#' {
            nachbar.push((x + 1, y));
        }
        if y > 0 && input[y - 1][x] != '#' {
            nachbar.push((x, y - 1));
        }
        if y < input.len() - 1 && input[y + 1][x] != '#' {
            nachbar.push((x, y + 1));
        }

        for v in nachbar {
            if Q.contains(&v) {
                let vor_opt = vorgaenger[u.0 + u.1 * input.len()];
                let weigth = match vor_opt {
                    Some(vor) => {
                        if (vor.0 as i32 - u.0 as i32, vor.1 as i32 - u.1 as i32)
                            == (u.0 as i32 - v.0 as i32, u.1 as i32 - v.1 as i32)
                        {
                            1
                        } else {
                            1001
                        }
                    }
                    None => 1,
                };

                let alternativ: usize = abstand[x + y * input.len()] + weigth;
                if alternativ < abstand[v.0 + v.1 * input.len()] {
                    abstand[v.0 + v.1 * input.len()] = alternativ;
                    vorgaenger[v.0 + v.1 * input.len()] = Some(u);
                }
            }
        }
    }

    let mut path = vec![end];

    let mut u = end;
    while vorgaenger[u.0 + u.1 * input.len()].is_some() {
        u = vorgaenger[u.0 + u.1 * input.len()].unwrap();
        path.push(u);
    }

    print_maze(&input, &path);
    println!();
    println!("{}", abstand[end.0 + end.1 * input.len()]);
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
