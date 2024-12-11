use regex::Regex;

fn main() {
    let re = Regex::new(r"XMAS").unwrap();
    let re2 = Regex::new(r"SAMX").unwrap();

    let input = utils::read_file_into_list("day4/puzzle1/list");

    let mut search_space = input.clone();
    // let mut search_space = Vec::new();

    let m = input[0].len();
    let n = input.len();

    for j in 0..m {
        let mut new_string = String::new();
        for i in 0..n {
            new_string.push(input[i].as_bytes()[j] as char);
        }
        search_space.push(new_string);
    } 

    for i in 0..m {
        let mut x = 0;
        let mut y = i;
        let mut new_string = String::new();
        while x < n && y < m {
            new_string.push(input[x].as_bytes()[y] as char);
            x += 1;
            y += 1;
        }
        search_space.push(new_string);
    }

    for i in 1..n {
        let mut x = i;
        let mut y = 0;
        let mut new_string = String::new();
        while x < n && y < m {
            new_string.push(input[x].as_bytes()[y] as char);
            x += 1;
            y += 1;
        }
        search_space.push(new_string);
    }



    for i in 0..m {
        let mut x = 0;
        let mut y = i;
        let mut new_string = String::new();
        while x < n {
            new_string.push(input[x].as_bytes()[y] as char);
            if y == 0 {
                break;
            }
            x += 1;
            y -= 1;
        }
        search_space.push(new_string);
    }

    for i in 1..n {
        let mut x = i;
        let mut y = m - 1;
        let mut new_string = String::new();
        while x < n {
            new_string.push(input[x].as_bytes()[y] as char);
            if y == 0 {
                break;
            }
            x += 1;
            y -= 1;
        }
        search_space.push(new_string);
    }

    let sum:usize = search_space.iter()
    .filter(|&x| x.len() > 3)
    .map(|x| re.find_iter(x.as_str()).count() +  re2.find_iter(x.as_str()).count())
    .sum();


    println!("{}", sum);
}
