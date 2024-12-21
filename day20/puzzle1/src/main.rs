use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
    vec,
};

use colored::Colorize;

fn main() {
    let mut g = utils::read_2d_grid("day20/puzzle1/input");
    let w = (g.len() as f64).sqrt() as i32;
    let s = find_index_of_letter(&g, 'S');
    let e = find_index_of_letter(&g, 'E');

    let mut path = vec![s];
    let mut saving_map = HashMap::new();

    while *path.last().unwrap() != e {
        let u = *path.last().unwrap() as i32;
        let (xu, yu) = (u % w, u / w);
        for dir in DIRS {
            let (xv, yv) = (xu + dir.0, yu + dir.1);
            if xv >= 0 && yv >= 0 && xv < w && yv < w {
                let v = (xv + yv * w) as usize;
                if g[v] != '#' && !path.contains(&v) {
                    path.push(v);
                    break;
                }
            }
        }
    }

    for (ui, &u) in path.iter().enumerate() {
        println!("{} - {}", ui, path.len());
        let (xu, yu) = (u as i32 % w, u as i32 / w);
        for cheat_cost in 2..21 as i32 {
            let mut n = HashSet::new();
            for (yv, xv) in (0..=cheat_cost).zip((0..=cheat_cost).rev()) {
                for (x, y) in [
                    (xu + xv, yu + yv),
                    (xu + xv, yu - yv),
                    (xu - xv, yu + yv),
                    (xu - xv, yu - yv),
                ] {
                    let v = (x + y * w) as usize;
                    if x >= 0 && x < w && y >= 0 && y < w && g[v as usize] != '#' {
                        n.insert(v);
                    }
                }
            }
            for v in n {
                let (vi, _) = path.iter().enumerate().find(|(_, &x)| v == x).unwrap();
                if vi > ui {
                    let saving = vi - ui;
                    if saving > cheat_cost as usize {
                        let saving = saving - 2;
                        if saving_map.contains_key(&saving) {
                            let temp = saving_map.get(&saving).unwrap();
                            saving_map.insert(saving, temp + 1);
                        } else {
                            saving_map.insert(saving, 1);
                        }
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for (saving, counter) in saving_map {
        println!("{} {}", saving, counter);
        if saving >= 100 {
            sum += counter;
        }
    }

    println!("{sum}");
}

static DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

fn find_index_of_letter(g: &Vec<char>, letter: char) -> usize {
    return g
        .iter()
        .enumerate()
        .find(|x| *x.1 == letter)
        .map(|x| x.0)
        .unwrap();
}

fn print_the_thing(g: &Vec<char>, path: &Vec<usize>) {
    let s = (g.len() as f64).sqrt() as usize;
    let mut str = String::new();
    for y in 0..s {
        for x in 0..s {
            if path.contains(&(x + y * s)) {
                str.push_str("*".blue().to_string().as_str());
            } else if g[x + y * s] == '#' {
                str.push_str("▓".red().to_string().as_str());
            } else {
                str.push('.');
            }
        }
        str.push('\n');
    }
    println!("{str}");
}
