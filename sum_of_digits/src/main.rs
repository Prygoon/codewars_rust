fn main() {
    println!("{:?}", digital_root(1234));
}

fn digital_root(n: i64) -> i64 {
    let mut result: Vec<i64> = Vec::new();
    let mut n_mut = n;
    // result.push();

    while n_mut > 0 {
        result.push(n_mut % 10);
        n_mut /= 10;
    }

    if result.len() > 1 {
        digital_root(result.iter().sum())
    } else {
        result.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
    }
}
