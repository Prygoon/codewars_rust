fn main() {
    println!("{}", evens_and_odds(124));
}

fn evens_and_odds(n: u64) -> String {
    match n % 2 {
        0 => format!("{:b}", n),
        _ => format!("{:x}", n),
    }
}

#[cfg(test)]
mod tests {
    use super::evens_and_odds;

    #[test]
    fn basic() {
        assert_eq!(evens_and_odds(2), "10");
        assert_eq!(evens_and_odds(13), "d");
    }
}
