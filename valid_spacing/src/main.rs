fn main() {
    println!("{:?}", valid_spacing(""));
}

fn valid_spacing(s: &str) -> bool {
    match s.len() {
        0 => true,
        _ => s.split(" ").filter(|x| x.is_empty()).count() == 0,
    }
}

#[cfg(test)]
mod tests {
    use super::valid_spacing;

    #[test]
    fn samples() {
        assert_eq!(valid_spacing("Hello world"), true, "Testing 'Hello world'");
        assert_eq!(
            valid_spacing(" Hello world"),
            false,
            "Testing ' Hello world'"
        );
        assert_eq!(
            valid_spacing("Hello  world "),
            false,
            "Testing 'Hello  world '"
        );
        assert_eq!(valid_spacing(""), true, "Testing ''");
        assert_eq!(valid_spacing(" "), false, "Testing ' '");
    }
}
