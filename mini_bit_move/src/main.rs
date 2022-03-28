fn main() {
    println!("{}", interpreter("10", "1010101"));
}

fn interpreter(tape: &str, data: &str) -> String {
    // your solution here
    data.chars().map(|c| c.to_digit(2));
    "".to_string()
}

#[test]
fn basic_tests() {
    // flip every bit
    assert_eq!(interpreter("10", "1010101"), "0101010");
    // flip every other bit
    assert_eq!(interpreter("100", "1111111111"), "0101010101");
}
