fn main() {
    println!("{}", min_value(vec![4, 7, 5, 7]));
}

fn min_value(mut digits: Vec<i32>) -> i32 {
    // let mut unique_vec = digits
    //     .into_iter()
    //     .collect::<HashSet<i32>>()
    //     .into_iter()
    //     .collect::<Vec<i32>>();
    //
    // unique_vec.sort_unstable();
    // unique_vec.into_iter().fold(0, |acc, num| (acc * 10 + num))
    digits.sort_unstable();
    digits.dedup();

    digits.into_iter().fold(0, |acc, x| acc * 10 + x)
}

#[test]
fn basic_test() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}
