fn main() {
    println!("{:?}", find_digit(213, 3));
}

fn find_digit(num: i32, nth: i32) -> i32 {
    if nth <= 0 {
        return -1;
    }

    let mut mut_num: i32 = num;

    for _ in 0..nth - 1 {
        mut_num = mut_num.abs() / 10;
    }
    mut_num % 10
}

#[test]
fn example_test() {
    assert_eq!(find_digit(5673, 4), 5);
    assert_eq!(find_digit(129, 2), 2);
    assert_eq!(find_digit(-2825, 3), 8);
    assert_eq!(find_digit(-456, 4), 0);
    assert_eq!(find_digit(0, 20), 0);
    assert_eq!(find_digit(65, 0), -1);
    assert_eq!(find_digit(24, -8), -1);
}
