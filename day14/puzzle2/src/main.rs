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
        save_to_jpg(&vec, w, h, iterations);
        iterations += 1;
        if iterations > 100000 {
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
                let image::Rgb(data) = *pixel;
                *pixel = rgb;
            }
        }
    }

    let mut s = "day14/puzzle2/pics/".to_owned();
    s.push_str(&iterations.to_string());
    s.push_str(&".png".to_owned());
    imgbuf
        .save(s.as_str())
        .unwrap();
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
