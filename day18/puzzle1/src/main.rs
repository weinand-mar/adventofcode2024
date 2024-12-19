use core::f64;

use colored::Colorize;

fn main() {
    let input = utils::read_file_into_list_of_cordinates("day18/puzzle1/input");
    let w: usize = 71;
    let mut g = vec!['.'; w * w];

    for &(x, y) in input.iter().take(1024) {
        g[x + y * w] = '#';
    }

    let s = 0;
    let e = w - 1 + (w - 1) * w;
    let vorgaenger = a_star(&g, s, e);
    let path = trace_path(&vorgaenger.unwrap(), e);
    println!("{}", path.len() - 1);

    for &(x, y) in input.iter().skip(1024) {
        g[x + y * w] = '#';
        if let None = a_star(&g, s, e) {
            println!("{x}, {y}");
            break;
        }
    }
}   

static DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

fn a_star(g: &Vec<char>, s: usize, e: usize) -> Option<Vec<Option<usize>>> {
    let w = (g.len() as f64).sqrt() as usize;
    let mut vorgaenger: Vec<Option<usize>> = vec![None; g.len()];
    let mut g_score: Vec<usize> = vec![usize::MAX - 5000; g.len()];
    let mut f_score: Vec<f64> = vec![f64::MAX - 5000.0; g.len()];
    
    g_score[s] = 0;
    f_score[s] = h(g, s, e);
    let mut q: Vec<usize> = vec![s];

    while !q.is_empty() {
        let (i, &u) = q.iter().enumerate()
        .min_by_key(|(i, &u)| f_score[u] as usize)
        .unwrap();

        q.remove(i);

        if u == e {
            return Some(vorgaenger);
        }

        let u_cord = (u % w, u / w);

        for dir in DIRS {
            let v_cord = (u_cord.0 as i32 + dir.0, u_cord.1 as i32 + dir.1);
            if v_cord.0 >= 0 && v_cord.1 >= 0 && v_cord.0 < w as i32 && v_cord.1 < w as i32 {
                let v = v_cord.0 as usize + v_cord.1 as usize * w;
                if g[v] != '#' {
                    let alt = g_score[u] + 1;
                    if g_score[v] == usize::MAX - 5000 && alt < g_score[v] {
                        vorgaenger[v] = Some(u);
                        g_score[v] = alt;
                        f_score[v] = alt as f64 + h(g, v, e);
                        if !q.contains(&v) {
                            q.push(v);
                        }
                    }
                }
            }
        }
    }
    return None;
}

fn h(g: &Vec<char>, s: usize, e: usize) -> f64 {
    let w = (g.len() as f64).sqrt() as i32;
    let s = s as i32;
    let e = e as i32;
    let (sx, sy) = (s % w, s/ w);
    let (ex, ey) = (e % w, e/ w);

    return (((sx - ex).pow(2) + (sy - ey).pow(2)) as f64).sqrt();
}


fn dijkstra(g: &Vec<char>, s: usize) -> Vec<Option<usize>> {
    let w = (g.len() as f64).sqrt() as usize;
    let mut vorgaenger: Vec<Option<usize>> = vec![None; g.len()];
    let mut abstand: Vec<usize> = vec![usize::MAX - 5000; g.len()];
    
    abstand[s] = 0;
    let mut q: Vec<usize> = Vec::new();
    for i in 0..g.len() {
        q.push(i);
    }

    while !q.is_empty() {
        let (i, &u) = q.iter().enumerate()
        .min_by_key(|(i, &u)| abstand[u])
        .unwrap();

        q.remove(i);

        let u_cord = (u % w, u / w);
        for dir in DIRS {
            let v_cord = (u_cord.0 as i32 + dir.0, u_cord.1 as i32 + dir.1);
            if v_cord.0 >= 0 && v_cord.1 >= 0 && v_cord.0 < w as i32 && v_cord.1 < w as i32 {
                let v = v_cord.0 as usize + v_cord.1 as usize * w;
                if g[v] != '#' {
                    let alt = abstand[u] + 1;
                    if alt < abstand[v] {
                        vorgaenger[v] = Some(u);
                        abstand[v] = alt;
                    }
                }
            }
        }
    }
    return vorgaenger;
}

fn trace_path(vorgaenger: &Vec<Option<usize>>, e: usize) -> Vec<usize> {
    let mut path = vec![e];
    let mut u_opt = vorgaenger[e];
    while let Some(u) = u_opt {
        path.push(u);
        u_opt = vorgaenger[u];
    }
    return path;
}

fn print_the_thing(g: &Vec<char>, path: &Vec<usize>) {
    let s = (g.len() as f64).sqrt() as usize;
    let mut str = String::new();
    for y in 0..s {
        for x in 0..s {
            if g[x + y * s] == '#' {
                str.push_str("▓".blue().to_string().as_str());
            }
            else if path.contains(&(x + y * s)) {
                str.push_str("*".red().to_string().as_str());
            }
            else {
                str.push('.');
            }
        }
        str.push('\n');
    }
    println!("{str}");
}
