use std::{
    collections::{HashMap, HashSet},
    usize,
};

use colored::Colorize;

fn main() {
    let input = utils::read_file_into_list_of_list_of_chars("day16/puzzle1/input");
    let start = search_letter(&input, 'S');
    let end = search_letter(&input, 'E');
    let s = start.0 + start.1 * input.len();
    let e = end.0 + end.1 * input.len();

    let mut G = map_g(&input);
    G[s].insert(((start.1 - 1) * input.len() + start.0), 1001);

    // let (abstand, vorgaenger) = dijkstra(&G, s);

    // let mut dist = vec![vec![0; G.len()]; G.len()];

    // for v in 0..G.len() {
    //     dist[v][v] = 0;
    // }

    // for k in 0..G.len() {
    //     for i in 0..G.len() {
    //         for j in 0..G.len() {
    //             if dist[i][j] > dist[i][k] + dist[k][j] {
    //                 dist[i][j] = dist[i][k] + dist[k][j];
    //             }
    //         }
    //     }
    // }

    // println!("{:?}", dist[s][e]);

    let mut abstand: Vec<usize> = vec![usize::MAX - 5000; G.len()];
    let mut vorgaenger: Vec<Vec<usize>> = vec![Vec::new(); G.len()];
    abstand[s] = 0;

    for _ in 0..G.len() - 1 {
        for u in 0..G.len() {
            for (&v, &w) in &G[u] {
                if abstand[u] + w < abstand[v] {
                    abstand[v] = abstand[u] + w;
                    vorgaenger[v] = vec![u];
                }
                else if abstand[u] + w == abstand[v] {
                    if !vorgaenger[v].contains(&u) {
                        vorgaenger[v].push(u);
                    }
                }
            }
        }
    }

    let mut paths = vec![vec![e]];
    let mut finished_paths = Vec::new();
    while !paths.is_empty() {
        let path = paths.pop().unwrap();
        for v in &vorgaenger[path[path.len() - 1]] {
            let mut cloned = path.clone();
            cloned.push(*v);
            paths.push(cloned);
        }
        if vorgaenger[path[path.len() - 1]].is_empty() {
            finished_paths.push(path);
        }
    }

    let collects: Vec<Vec<(usize, usize)>> = finished_paths
        .into_iter()
        .map(|path| {
            path.into_iter()
                .map(|x| (x % input.len(), x / input.len()))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let mut new_paths = Vec::new();
    for collect in collects {
        let mut new_path = Vec::new();
        for i in 1..collect.len() {
            let u = collect[i - 1];
            let v = collect[i];
            let d = (u.0 as i32 - v.0 as i32, u.1 as i32 - v.1 as i32);
            new_path.push(u);
            if d.0.abs() == 2 || d.1.abs() == 2 {
                if d.0 == 0 || d.1 == 0 {
                    new_path.push((
                        (u.0 as i32 - d.0 / 2) as usize,
                        (u.1 as i32 - d.1 / 2) as usize,
                    ));
                }
            } else if d.0.abs() == 1 && d.1.abs() == 1 {
                if (v.1 as i32 + d.1) >= 0
                    && (v.1 as i32 + d.1) < input.len() as i32
                    && input[(v.1 as i32 + d.1) as usize][v.0] != '#'
                {
                    new_path.push((v.0, (v.1 as i32 + d.1) as usize));
                } else {
                    new_path.push(((v.0 as i32 + d.0) as usize, v.1));
                }
            }
            new_path.push(v);
        }
        new_paths.push(new_path);
    }

    let mut visited = HashSet::new();
    print_maze(&input, &new_paths);
    for path in &new_paths {
        for v in path {
            visited.insert(*v);
        }
    }
    println!("{}", abstand[e]);
    println!("{}", visited.len());
}

fn dijkstra(G: &Vec<HashMap<usize, usize>>, s: usize) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut abstand: Vec<usize> = vec![usize::MAX - 5000; G.len()];
    let mut vorgaenger: Vec<Vec<usize>> = vec![Vec::new(); G.len()];
    let mut Q: Vec<usize> = Vec::new();
    for y in 0..G.len() {
        Q.push(y);
    }
    abstand[s] = 0;
    while !Q.is_empty() {
        let (i, &u) = Q
            .iter()
            .enumerate()
            .min_by(|&a, &b| abstand[*a.1].cmp(&abstand[*b.1]))
            .unwrap();

        for (&v, &w) in G[u].iter() {
            if Q.contains(&v) {
                let alt = abstand[u] + w;
                if alt < abstand[v] {
                    abstand[v] = alt;
                    vorgaenger[v] = vec![u];
                }
                if alt == abstand[v] {
                    if !vorgaenger[v].contains(&u) {
                        vorgaenger[v].push(u);
                    }
                }
            }
        }
        Q.remove(i);
    }
    return (abstand, vorgaenger);
}

fn print_maze(input: &Vec<Vec<char>>, paths: &Vec<Vec<(usize, usize)>>) {
    // clearscreen::clear().unwrap();
    let mut s = String::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let mut contains = false;
            for path in paths {
                if path.contains(&(x, y)) {
                    s.push_str(String::from('*').red().to_string().as_str());
                    contains = true;
                    break;
                }
            }
            if contains {
            } else if input[y][x] == '#' {
                s.push_str(String::from('▓').blue().to_string().as_str());
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

fn map_g(input: &Vec<Vec<char>>) -> Vec<HashMap<usize, usize>> {
    let mut G: Vec<HashMap<usize, usize>> = vec![HashMap::new(); input.len() * input[0].len()];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == '.' || input[y][x] == 'E' || input[y][x] == 'S' {
                let u = x + y * input.len();
                let mut n = Vec::new();
                let mut n2 = Vec::new();
                if x > 0 && input[y][x - 1] != '#' {
                    n.push(x - 1 + y * input.len());
                    n2.push((x - 1, y));
                }
                if x < input.len() - 1 && input[y][x + 1] != '#' {
                    n.push(x + 1 + y * input.len());
                    n2.push((x + 1, y));
                }
                if y > 0 && input[y - 1][x] != '#' {
                    n.push(x + (y - 1) * input.len());
                    n2.push((x, y - 1));
                }
                if y < input.len() - 1 && input[y + 1][x] != '#' {
                    n.push(x + (y + 1) * input.len());
                    n2.push((x, y + 1));
                }

                if n.len() == 1 {
                    G[u].insert(n[0], 1);
                } else if n.len() == 2 {
                    if n2[0].0 as i32 - x as i32 == x as i32 - n2[1].0 as i32
                        && n2[0].1 as i32 - y as i32 == y as i32 - n2[1].1 as i32
                    {
                        let one = 1;
                        let zero = 0;
                        let weigth1 = *G[u].get(&n[0]).or(Some(&zero)).unwrap().max(&one);
                        let weigth2 = *G[u].get(&n[1]).or(Some(&zero)).unwrap().max(&one);
                        G[u].insert(n[0], weigth1);
                        G[u].insert(n[1], weigth2);
                    } else {
                        let one = 1;
                        let zero = 0;
                        let weigth1 = *G[n[0]].get(&u).or(Some(&zero)).unwrap().max(&one);
                        let weigth2 = *G[n[1]].get(&u).or(Some(&zero)).unwrap().max(&one);
                        G[n[0]].insert(u, weigth1);
                        G[n[1]].insert(u, weigth2);
                        G[u].insert(n[0], 1001);
                        G[u].insert(n[1], 1001);
                    }
                } else if n.len() == 3 {
                    let mut k: usize = 0;
                    let mut l: usize = 0;
                    let mut s: usize = 0;
                    for i in 0..n.len() {
                        for j in 0..n.len() {
                            if i != j {
                                for e in 0..n.len() {
                                    if e != i && e != j {
                                        if n2[i].0 as i32 - x as i32 == x as i32 - n2[j].0 as i32
                                            && n2[i].1 as i32 - y as i32
                                                == y as i32 - n2[j].1 as i32
                                        {
                                            k = n[i];
                                            l = n[j];
                                            s = n[e];
                                        }
                                    }
                                }
                            }
                        }
                    }

                    G[u].insert(k, 1);
                    G[u].insert(l, 1);
                    G[u].insert(s, 1001);
                    let one = 1;
                    let zero = 0;
                    let weigth1 = *G[k].get(&u).or(Some(&zero)).unwrap().max(&one);
                    let weigth2 = *G[l].get(&u).or(Some(&zero)).unwrap().max(&one);
                    G[k].insert(u, weigth1);
                    G[l].insert(u, weigth2);
                    G[s].insert(u, 1001);
                } else if n.len() == 4 {
                    for i in 0..n.len() {
                        for j in 0..n.len() {
                            if i != j {
                                if n2[i].0 as i32 - x as i32 == x as i32 - n2[j].0 as i32
                                    && n2[i].1 as i32 - y as i32 == y as i32 - n2[j].1 as i32
                                {
                                    G[n[i]].insert(n[j], 2);
                                    G[n[j]].insert(n[i], 2);
                                } else {
                                    G[n[i]].insert(n[j], 1002);
                                    G[n[j]].insert(n[i], 1002);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return G;
}
