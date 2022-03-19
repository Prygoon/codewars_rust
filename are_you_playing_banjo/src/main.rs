fn main() {
    println!("{}", are_you_playing_banjo("Martin"));
}

fn are_you_playing_banjo(name: &str) -> String {
    match &name[0..1] {
        "R" | "r" => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name)
    }
}


// fn are_you_playing_banjo(name: &str) -> String {
//     let char: char = name.chars().nth(0).unwrap().to_ascii_lowercase();
//     let result: &str;
//
//     if char == 'r' {
//         result = " plays";
//     } else {
//         result = " does not play";
//     }
//
//     name.to_string() + result + " banjo"
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}
