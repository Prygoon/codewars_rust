fn main() {
    greet();
}

fn greet() -> &'static str {
    return "hello world!"
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greets_the_world() {
        assert_eq!(greet(), "hello world!", "should return the correct message");
    }
}