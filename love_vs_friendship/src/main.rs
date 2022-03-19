fn main() {
    println!("{}", words_to_marks("hellowordsfgdgывавыld"));
}

fn words_to_marks(s: &str) -> u32 {
    s.chars().map(|char| (char as u32 - 96)).sum()
}

#[cfg(test)]
mod tests {
    use super::words_to_marks;

    #[test]
    fn basic() {
        assert_eq!(words_to_marks("attitude"), 100);
        assert_eq!(words_to_marks("friends"), 75);
        assert_eq!(words_to_marks("family"), 66);
        assert_eq!(words_to_marks("selfness"), 99);
        assert_eq!(words_to_marks("knowledge"), 96);
    }
}
