fn main() {
    println!("{:?}", gimme([-2, -3, -1]));
}

fn gimme(input_array: [i32; 3]) -> usize {
    let mut enum_vec: Vec<(usize, &i32)> = input_array.iter().enumerate().collect();

    enum_vec.sort_by_key(|k| k.1);
    enum_vec[1].0
}

#[cfg(test)]
mod tests {
    use super::gimme;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }
}
