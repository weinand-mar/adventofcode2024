use std::fs;
pub fn read_file_into_list_tuple(path: &str) -> (Vec<i32>, Vec<i32>) {
    let read_to_string = fs::read_to_string(path)
    .expect("Error");

    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    for line in read_to_string.lines() {
        let split = line.split_whitespace();
        let collect = split.collect::<Vec<&str>>();
        l1.push(collect[0].parse::<i32>().unwrap());
        l2.push(collect[1].parse::<i32>().unwrap());
    }

    l1.sort();
    l2.sort();
    return (l1, l2);
}

pub fn read_file_into_list(path: &str) -> Vec<String> {
    let read_to_string = fs::read_to_string(path).unwrap();

    return read_to_string.lines()
    .into_iter()
    .map(|x| String::from(x))
    .collect();
}

pub fn read_file_into_list_of_cordinates(path: &str) -> Vec<(usize, usize)> {
    let read_to_string = fs::read_to_string(path).unwrap();

    return read_to_string.lines()
    .into_iter()
    .map(|x| x.split(",").collect::<Vec<&str>>())
    .map(|x| (x[0].parse::<usize>().unwrap(), x[1].parse::<usize>().unwrap()))
    .collect();
}

pub fn read_file_into_list_of_list(path: &str) -> Vec<Vec<i32>> {
    let read_to_string = fs::read_to_string(path).unwrap();

    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in read_to_string.lines() {
        let split = line.split_whitespace();
        let collect = split.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        result.push(collect);
    }

    return result;
}

pub fn read_file_into_list_of_list_of_numbers(path: &str) -> Vec<Vec<i32>> {
    let read_to_string = fs::read_to_string(path).unwrap();

    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in read_to_string.lines() {
        let split = line.as_bytes().iter();
        let collect = split.map(|x| (*x - '0' as u8) as i32 ).collect::<Vec<i32>>();
        result.push(collect);
    }

    return result;
}

pub fn read_file_into_list_of_list_of_chars(path: &str) -> Vec<Vec<char>> {
    let read_to_string = fs::read_to_string(path).unwrap();

    return read_to_string.lines().into_iter()
    .map(|x| x.as_bytes().iter().map(|&x| x as char).collect())
    .collect();
}