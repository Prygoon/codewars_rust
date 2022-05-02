fn main() {
    println!("{:?}", digitize(45657));
}

///Given a random non-negative number, you have to return the digits of this number within an array in reverse order.
///Example:
///
///348597 => [7,9,5,8,4,3]
///0 => [0]
fn digitize(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::digitize;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(digitize(0), vec![0]);
    }
}
