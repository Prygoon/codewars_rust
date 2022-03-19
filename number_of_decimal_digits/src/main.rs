use std::iter::successors;

fn main() {
    println!("{:?}", digits(1233));
}

fn digits(n: u64) -> usize {
    successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count()
}

#[test]
fn sample_test() {
    assert_eq!(digits(5), 1);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(9876543210), 10);
}
