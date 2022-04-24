fn main() {
    println!("{}", opposite(34));
}

/// Very simple, given an integer or a floating-point number, find its opposite.
///
/// Examples:
///
/// 1: -1
/// 14: -14
/// -34: 34
fn opposite(number: i32) -> i32 {
    -number
}

#[test]
fn returns_expected() {
    assert_eq!(opposite(1), -1);
    assert_eq!(opposite(-1), 1);
}
