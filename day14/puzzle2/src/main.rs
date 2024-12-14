use std::collections::HashSet;

struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}
fn main() {
    let mut robots = utils::read_file_into_list("day14/puzzle2/input")
        .iter()
        .map(|x| map_to_robot(x))
        .collect::<Vec<Robot>>();

    let w: usize = 101;
    let h: usize = 103;

    let mut vec = vec![0; w * h];
    for r in robots.iter() {
        vec[r.p.0 as usize + r.p.1 as usize * w] += 1
    }

    let mut iterations = 1;
    loop {
        for r in robots.iter_mut() {
            vec[r.p.0 as usize + r.p.1 as usize * w] -= 1;

            let mut new_x = (r.p.0 + r.v.0) % w as i32;
            let mut new_y = (r.p.1 + r.v.1) % h as i32;

            if new_x < 0 {
                new_x += w as i32;
            }
            if new_y < 0 {
                new_y += h as i32;
            }
            r.p = (new_x, new_y);

            vec[r.p.0 as usize + r.p.1 as usize * w] += 1;
        }
        if (iterations - 8) % 101 == 0 || (iterations - 78) % 103 == 0 {
            let num_components = components(&vec, w, h).len();
            if num_components < 200 {
                println!("{}, {}", iterations, num_components);
                save_to_jpg(&vec, w, h, iterations);
            }
        }
        iterations += 1;
        if iterations > 10000000 {
            break;
        }
    }
}

fn save_to_jpg(vec: &Vec<i32>, w: usize, h: usize, iterations: i32) {
    let mut imgbuf = image::ImageBuffer::new(w as u32, h as u32);
    let rgb = image::Rgb([255 as u8, 255 as u8, 255 as u8]);

    for i in 0..h {
        for j in 0..w {
            if vec[i * w + j] == 0 {
            } else {
                let pixel = imgbuf.get_pixel_mut(j as u32, i as u32);
                // let image::Rgb(data) = *pixel;
                *pixel = rgb;
            }
        }
    }

    let mut s = "day14/puzzle2/pics/".to_owned();
    s.push_str(&iterations.to_string());
    s.push_str(&".bmp".to_owned());
    imgbuf.save(s.as_str()).unwrap();
}

fn map_to_robot(str: &String) -> Robot {
    let i = str.find('v').unwrap();
    let mut p = str[..i].trim();
    let mut v = str[i..].trim();
    p = p[p.find('=').unwrap() + 1..].trim();
    v = v[v.find('=').unwrap() + 1..].trim();
    let split = p
        .split(',')
        .map(|x: &str| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let split2 = v
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    return Robot {
        p: (split[0], split[1]),
        v: (split2[0], split2[1]),
    };
}

fn components(input: &Vec<i32>, w: usize, h: usize) -> Vec<HashSet<(usize, usize)>> {
    let mut components: Vec<HashSet<(usize, usize)>> = Vec::new();
    let mut global_visited: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..w {
        for j in 0..h {
            if input[i + j * w] > 0 && !global_visited.contains(&(i, j)) {
                let component = dfs(input, w, h, (i, j));
                for p in component.iter() {
                    global_visited.insert(p.clone());
                }
                components.push(component);
            }
        }
    }
    return components;
}

fn dfs(input: &Vec<i32>, w: usize, h: usize, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut stack = vec![start];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while !stack.is_empty() {
        let item = stack.pop().unwrap();
        visited.insert(item);

        if item.0 > 0 && input[item.0 - 1 + item.1 * w] > 0 && !visited.contains(&(item.0 - 1, item.1)){
            stack.push((item.0 - 1, item.1));
        }
        if item.0 < w - 1 && input[item.0 + 1 + item.1 * w] > 0 && !visited.contains(&(item.0 + 1, item.1)) {
            stack.push((item.0 + 1, item.1));
        }
        if item.1 > 0 && input[item.0 + (item.1 - 1) * w] > 0 && !visited.contains(&(item.0, item.1 - 1)) {
            stack.push((item.0, item.1 - 1));
        }
        if item.1 < h - 1 && input[item.0 + (item.1 + 1) * w] > 0 && !visited.contains(&(item.0, item.1 + 1)) {
            stack.push((item.0, item.1 + 1));
        }
    }
    return visited;
}
