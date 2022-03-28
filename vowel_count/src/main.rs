fn main() {
    println!("{:?}", get_count("sjdfhsoidrj"));
}

/// Return the number (count) of vowels in the given string.
///
/// We will consider a, e, i, o, u as vowels for this Kata (but not y).
///
/// The input string will only consist of lower case letters and/or spaces.
fn get_count(string: &str) -> usize {
    string.bytes().filter(|b| b"eaiou".contains(b)).count()
}

#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}
