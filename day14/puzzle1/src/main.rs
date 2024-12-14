struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}
fn main() {
    let mut input = utils::read_file_into_list("day14/puzzle1/input")
        .iter()
        .map(|x| map_to_robot(x))
        .collect::<Vec<Robot>>();

    let w: usize = 101;
    let h: usize = 103;

    let iterations = 6876;
    for r in input.iter_mut() {
        let mut new_x = (r.p.0 + r.v.0 * iterations as i32) % w as i32;
        let mut new_y = (r.p.1 + r.v.1 * iterations as i32) % h as i32;

        if new_x < 0 {
            new_x += w as i32;
        }
        if new_y < 0 {
            new_y += h as i32;
        }
        r.p = (new_x, new_y);
    }

    print_map(&input, w, h);
    println!();

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut sum3 = 0;
    let mut sum4 = 0;
    for r in input {
        // links
        if r.p.0 < w as i32 / 2 {
            if r.p.1 < h as i32 / 2 {
                sum1 += 1;
            } else if r.p.1 > h as i32 / 2 {
                sum2 += 1;
            }
        } else if r.p.0 > w as i32 / 2 {
            if r.p.1 < h as i32 / 2 {
                sum3 += 1;
            } else if r.p.1 > h as i32 / 2 {
                sum4 += 1;
            }
        }
    }
    println!("{sum1}, {sum2}, {sum3}, {sum4}");
    let result = sum1 * sum2 * sum3 * sum4;
    println!("{result}");
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

fn print_map(robots: &Vec<Robot>, w: usize, h: usize) {
    let mut vec = vec![0; w * h];

    for r in robots {
        print!("{:?}", r.p);
        vec[r.p.0 as usize + r.p.1 as usize * w] += 1
    }
    println!();

    for i in 0..h {
        for j in 0..w {
            if vec[i * w + j] == 0 {
                print!(".");
            } else {
                print!("{}", vec[i * w + j]);
            }
        }
        println!();
    }
}
