fn main() {
    println!("{}", longest("aretheyhere", "yestheyarehere"));
}

/// Take 2 strings s1 and s2 including only letters from ato z. Return a new sorted string, the longest possible, containing distinct letters - each taken only once - coming from s1 or s2.
/// Examples:
///
/// a = "xyaabbbccccdefww"
/// b = "xxxxyyyyabklmopq"
/// longest(a, b) -> "abcdefklmopqwxy"
///
/// a = "abcdefghijklmnopqrstuvwxyz"
/// longest(a, a) -> "abcdefghijklmnopqrstuvwxyz"
fn longest(a1: &str, a2: &str) -> String {
    let str = format!("{}{}", a1, a2);
    let mut vec = str.chars().collect::<Vec<_>>();

    vec.sort_unstable();
    vec.dedup();

    vec.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
    }
}
