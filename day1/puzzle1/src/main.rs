fn main() {
    let (l1, l2) = utils::read_file_into_list_tuple("list");

    let mut sum: i32 = 0;
    for i in 0..l1.len() {
        sum += (l1[i] - l2[i]).abs();
    }

    println!("{}", sum);
}


