use std::cmp::Reverse;

fn main() {
    println!("{:?}", descending_order(3245));
}

fn descending_order(x: u64) -> u64 {
    let mut result = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    result.sort_by_key(|w| Reverse(*w));
    result.iter().fold(0, |acc, num| (acc * 10 + (*num as u64)))
}
#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
