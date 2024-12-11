fn main() {
    let input = utils::read_file_into_list_of_list_of_chars("day8/puzzle2/input");
    let mut antinode = input.clone();

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let antena_type = input[i][j];
            if antena_type != '.' {
                for x in 0..input.len() {
                    for y in 0..input[x].len() {
                        if x != i && y != j && input[x][y] == input[i][j]  {
                            let dx = i as i32 - x as i32;
                            let dy = j as i32 - y as i32;

                            let mut a1x = i as i32 + dx;
                            let mut a1y = j as i32 + dy;

                            let mut a2x = x as i32 - dx;
                            let mut a2y = y as i32 - dy;

                            while a1x >= 0 && a1x < input.len() as i32 && a1y >= 0 && a1y < input[x].len() as i32 {
                                antinode[a1x as usize][a1y as usize] = '#';
                                a1x += dx;
                                a1y += dy;
                            }

                            while a2x >= 0 && a2x < input.len() as i32 && a2y >= 0 && a2y < input[x].len() as i32 {
                                antinode[a2x as usize][a2y as usize] = '#';
                                a2x -= dx;
                                a2y -= dy;
                            }
                            antinode[i][j] = '#';
                            antinode[x][y] = '#';
                        }
                    }
                }
            }
        }
    }

    for i in &antinode {
        for j in i {
            print!("{j}");
        }
        println!();
    }

    let sum = antinode.iter()
        .flat_map(|x| x.iter())
        .filter(|&&x| x == '#')
        .count();

    println!("{sum}")
}
