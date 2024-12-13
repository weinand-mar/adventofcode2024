use std::collections::{HashMap, HashSet};
use rand::Rng; // 0.8.5
fn main() {
    let fields = utils::read_file_into_list_of_list_of_chars("day12/puzzle1/input");
    let mut cluster: HashMap<char, Vec<HashSet<(usize, usize)>>> = HashMap::new();

    for x in 0..fields.len() {
        for y in 0..fields[x].len() {
            let prop_type = fields[x][y];
            if !cluster.contains_key(&prop_type) {
                cluster.insert(prop_type, vec![HashSet::from([(x, y)])]);
            } else {
                let field_clusters = cluster.get_mut(&prop_type).unwrap();
                let mut existing_clusters = Vec::new();
                // for field_cluster in field_clusters.iter_mut() {
                for i in 0..field_clusters.len() {
                    if is_prop_in_cluster((x, y), &field_clusters[i], &fields) {
                        field_clusters[i].insert((x, y));
                        existing_clusters.push(i);
                    }
                }
                if existing_clusters.is_empty() {
                    field_clusters.push(HashSet::from([(x, y)]));
                } else {
                    let (a, b) = field_clusters.split_at_mut(existing_clusters[0] + 1);
                    let alen = a.len();
                    let c = &mut a[existing_clusters[0]];
                    for i in 1..existing_clusters.len() {
                        let cn = &b[existing_clusters[i] - alen];
                        for item in cn {
                            c.insert(item.clone());
                        }
                    }

                    for i in 1..existing_clusters.len() {
                        field_clusters.remove(existing_clusters[i] - (i - 1));
                    }
                }
            }
        }
    }
    let mut sum_total = 0;
    for (prop_type, clusters) in cluster.iter() {
        for cluster in clusters {
            let mut sum = 0;
            for item in cluster {
                sum += count_non_same_neightbour(item, &fields, *prop_type);
            }
            println!(
                "{}: {} * {} = {}",
                prop_type,
                cluster.len(),
                sum,
                cluster.len() * sum
            );
            sum_total += cluster.len() * sum;
        }
    }

    let mut total_sum_2 = 0;
    for (prop_type, clusters) in cluster.iter() {
        for cluster in clusters {
            let mut collect = cluster.iter().collect::<Vec<&(usize, usize)>>();
            collect.sort_by(|a, b| {
                if a.0 != b.0 {
                    a.0.cmp(&b.0)
                } else {
                    a.1.cmp(&b.1)
                }
            });

            let mut sum_top = 0;
            let mut sum_bottom = 0;
            let mut sum_left = 0;
            let mut sum_right = 0;
            for (i, (x, y)) in collect.iter().enumerate() {
                // Nur die felder mit Zaun
                if *x == 0 || fields[*x - 1][*y] != *prop_type {
                    // Ist der letzte Eintrag
                    if i == collect.len() - 1 {
                        sum_top += 1;
                    } else {
                        let (next_x, next_y) = collect[i + 1];
                        // Ist in neuer Zeile
                        if *x != *next_x {
                            sum_top += 1;
                        }
                        // Das nächste Element in der selben Zeile is kein direkter Nachbar
                        else if *y + 1 != *next_y {
                            sum_top += 1;
                        }
                        // Das nächste Element hat keinen Zaun
                        else if *next_x != 0 && fields[*next_x - 1][*next_y] == *prop_type {
                            sum_top += 1;
                        }
                    }
                }

                 // Nur die felder mit Zaun
                 if *x == fields.len() - 1 || fields[*x + 1][*y] != *prop_type {
                    // Ist der letzte Eintrag
                    if i == collect.len() - 1 {
                        sum_bottom += 1;
                    } else {
                        let (next_x, next_y) = collect[i + 1];
                        // Ist in neuer Zeile
                        if *x != *next_x {
                            sum_bottom += 1;
                        }
                        // Das nächste Element in der selben Zeile is kein direkter Nachbar
                        else if *y + 1 != *next_y {
                            sum_bottom += 1;
                        }
                        // Das nächste Element hat keinen Zaun
                        else if *next_x != fields.len() - 1 && fields[*next_x + 1][*next_y] == *prop_type {
                            sum_bottom += 1;
                        }
                    }
                }
            }

            collect.sort_by(|a, b| {
                if a.1 != b.1 {
                    a.1.cmp(&b.1)
                } else {
                    a.0.cmp(&b.0)
                }
            });

            for (i, (x, y)) in collect.iter().enumerate() {
                // Nur die felder mit Zaun
                if *y == 0 || fields[*x][*y - 1] != *prop_type {
                    // Ist der letzte Eintrag
                    if i == collect.len() - 1 {
                        sum_left += 1;
                    } else {
                        let (next_x, next_y) = collect[i + 1];
                        // Ist in neuer Zeile
                        if *x + 1 != *next_x {
                            sum_left += 1;
                        }
                        // Das nächste Element in der selben Zeile is kein direkter Nachbar
                        else if *y != *next_y {
                            sum_left += 1;
                        }
                        // Das nächste Element hat keinen Zaun
                        else if *next_y != 0 && fields[*next_x][*next_y - 1] == *prop_type {
                            sum_left += 1;
                        }
                    }
                }

                 // Nur die felder mit Zaun
                if *y == fields.len() - 1 || fields[*x][*y + 1] != *prop_type {
                    // Ist der letzte Eintrag
                    if i == collect.len() - 1 {
                        sum_right += 1;
                    } else {
                        let (next_x, next_y) = collect[i + 1];
                        // Ist in neuer Zeile
                        if *x + 1 != *next_x {
                            sum_right += 1;
                        }
                        // Das nächste Element in der selben Zeile is kein direkter Nachbar
                        else if *y != *next_y {
                            sum_right += 1;
                        }
                        // Das nächste Element hat keinen Zaun
                        else if *next_y != fields.len() - 1 && fields[*next_x][*next_y + 1] == *prop_type {
                            sum_right += 1;
                        }
                    }
                }
            }

            println!();
            println!("{prop_type}: {sum_top} {sum_right} {sum_bottom} {sum_left}");
            println!("{:?}", collect);
            total_sum_2 += (sum_bottom + sum_top + sum_left + sum_right) * collect.len();
        }
    }
    println!("Part1 = {sum_total}");
    println!("Part2 = {total_sum_2}");
    visualize(&cluster, &fields);
}

fn visualize(cluster: &HashMap<char, Vec<HashSet<(usize, usize)>>>, fields: &Vec<Vec<char>>) {
    let mut imgbuf = image::ImageBuffer::new(fields.len() as u32 * 10, fields[0].len() as u32 * 10);
    for asdf in cluster {
        let rgb = image::Rgb([rand::thread_rng().gen_range(0..255) as u8, rand::thread_rng().gen_range(0..255) as u8, rand::thread_rng().gen_range(0..255) as u8]);
        for clu in asdf.1 {
            for &pos in clu {
                let (x, y) = pos;
                for i in 0..10 {
                    for j in 0..10 {
                        let pixel = imgbuf.get_pixel_mut(x as u32 * 10 + i, y as u32 * 10 + j);
                        let image::Rgb(data) = *pixel;
                        *pixel = rgb;
                    }
                }
            }
        }
    }
    imgbuf.save("fractal.png").unwrap();
}


fn count_non_same_neightbour(
    pos: &(usize, usize),
    fields: &Vec<Vec<char>>,
    prop_type: char,
) -> usize {
    let &(x, y) = pos;
    let mut sum = 0;
    if x == fields.len() - 1 || fields[x + 1][y] != prop_type {
        sum += 1;
    }
    if x == 0 || fields[x - 1][y] != prop_type {
        sum += 1;
    }
    if y == fields[x].len() - 1 || fields[x][y + 1] != prop_type {
        sum += 1;
    }
    if y == 0 || fields[x][y - 1] != prop_type {
        sum += 1;
    }
    return sum;
}

fn is_prop_in_cluster(
    pos: (usize, usize),
    field_cluster: &HashSet<(usize, usize)>,
    fields: &Vec<Vec<char>>,
) -> bool {
    let (x, y) = pos;
    let prop_type = fields[x][y];
    return x < fields.len() - 1
        && fields[x + 1][y] == prop_type
        && field_cluster.contains(&(x + 1, y))
        || x > 0 && fields[x - 1][y] == prop_type && field_cluster.contains(&(x - 1, y))
        || y < fields[x].len() - 1
            && fields[x][y + 1] == prop_type
            && field_cluster.contains(&(x, y + 1))
        || y > 0 && fields[x][y - 1] == prop_type && field_cluster.contains(&(x, y - 1));
}