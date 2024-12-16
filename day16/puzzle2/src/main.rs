use colored::Colorize;
use std::{
    collections::{HashMap, HashSet, LinkedList},
    io::stdin,
    path, usize, vec,
};

fn main() {
    let input = utils::read_file_into_list_of_list_of_chars("day16/puzzle1/input");
    let start = search_letter(&input, 'S');
    let end = search_letter(&input, 'E');

    let mut abstand: Vec<usize> = vec![usize::MAX - 5000; input.len() * input[0].len()];
    let mut vorgaenger: Vec<Vec<(usize, usize)>> = vec![vec![]; input.len() * input[0].len()];
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
                let vors_u = vorgaenger[u.0 + u.1 * input.len()].clone();
                for &vor_u in vors_u.iter() {
                    let weigth = if (vor_u.0 as i32 - u.0 as i32, vor_u.1 as i32 - u.1 as i32)
                        == (u.0 as i32 - v.0 as i32, u.1 as i32 - v.1 as i32)
                    {
                        1
                    } else {
                        1001
                    };

                    let alternativ: usize = abstand[x + y * input.len()] + weigth;
                    if v == (15, 7) {
                        println!("{alternativ}, {}", abstand[v.0 + v.1 * input.len()]);
                    }
                    if alternativ <= abstand[v.0 + v.1 * input.len()]
                        || alternativ - 1000 == abstand[v.0 + v.1 * input.len()]
                    {
                        abstand[v.0 + v.1 * input.len()] = alternativ;
                        let mut vors_v = &vorgaenger[v.0 + v.1 * input.len()];
                        let mut push = vors_v.clone();
                        // .iter()
                        // .map(|&x| x)
                        // .filter(|k| {
                        //     abstand[x + y * input.len()] == abstand[k.0 + k.1 * input.len()] || abstand[x + y * input.len()] == abstand[k.0 + k.1 * input.len()] -1000
                        // })
                        // .collect::<Vec<(usize, usize)>>();
                        if !push.contains(&u) {
                            push.push(u);
                        }
                        vorgaenger[v.0 + v.1 * input.len()] = push;
                    }
                }
                if vors_u.is_empty() {
                    let alternativ: usize = abstand[x + y * input.len()]
                        + if (-1, 0) == (u.0 as i32 - v.0 as i32, u.1 as i32 - v.1 as i32) {
                            1
                        } else {
                            1001
                        };
                    if v == (15, 7) {
                        println!("{alternativ}, {}", abstand[v.0 + v.1 * input.len()]);
                    }
                    if alternativ <= abstand[v.0 + v.1 * input.len()]
                        || alternativ + 1000 <= abstand[v.0 + v.1 * input.len()]
                    {
                        abstand[v.0 + v.1 * input.len()] = alternativ;
                        vorgaenger[v.0 + v.1 * input.len()] = vec![u];
                    }
                }
            }
        }
    }

    let mut paths = vec![vec![end]];
    let mut finished_paths = Vec::new();

    while !paths.is_empty() {
        let path = paths.pop().unwrap();
        let u = path[path.len() - 1];
        println!("{:?}, {:?}", u, vorgaenger[u.0 + u.1 * input.len()]);
        if vorgaenger[u.0 + u.1 * input.len()].is_empty() {
            finished_paths.push(path);
        } else {
            for &vor in vorgaenger[u.0 + u.1 * input.len()].iter() {
                let mut cloned_path = path.clone();
                cloned_path.push(vor);
                paths.push(cloned_path);
            }
        }
    }
    for path in finished_paths {
        print_maze(&input, &path);
        println!("{:?}", path);
    }
    println!("{}", abstand[end.0 + end.1 * input.len()]);
    println!("{}", abstand[15 + 8 * input.len()]);
    println!("{}", abstand[14 + 7 * input.len()]);
}

fn print_maze(input: &Vec<Vec<char>>, path: &Vec<(usize, usize)>) {
    // clearscreen::clear().unwrap();
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
