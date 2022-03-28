fn main() {
    println!("{:?}", solve("())()))))()()("));
}

fn solve(s: &str) -> Option<usize> {
    if s.len() % 2 == 1 {
        return None;
    }

    let mut open = 0;
    let mut close = 0;

    for b in s.bytes() {
        if b == b'(' {
            open += 1;
        } else if open == 0 {
            close += 1;
        } else {
            open -= 1;
        }
    }

    Some(open / 2 + open % 2 + close / 2 + close % 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve(")()("), Some(2));
        assert_eq!(solve("((()"), Some(1));
        assert_eq!(solve("((("), None);
        assert_eq!(solve("())((("), Some(3));
        assert_eq!(solve("())()))))()()("), Some(4));
    }
}
