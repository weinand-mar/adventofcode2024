fn main() {
    let (l1, l2) = utils::read_file_into_list_tuple("list");

    let mut sum: i32 = 0;
    for i in l1 {
        let count = l2.iter().filter(|&n| *n == i).count() as i32;
        sum += i * count;
    }

    print!("{}", sum);
}

