fn main() {
    println!("{:?}", expanded_form(1023));
}

fn expanded_form(n: u64) -> String {
    n.to_string()
        .chars()
        .enumerate()
        .filter(|c| c.1 != '0')
        .map(|(num, c)| format!("{}{}", c, "0".repeat(n.to_string().len() - num - 1)))
        .collect::<Vec<String>>()
        .join(" + ")
}

#[cfg(test)]
mod tests {
    use super::expanded_form;

    #[test]
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}
