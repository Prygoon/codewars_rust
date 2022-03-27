fn main() {
    println!("{:?}", multiplication_table(5));
}

fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    for i in 1..=len {
        let x = (1..=len).into_iter().map(|x| x * i).collect::<Vec<usize>>();
        result.push(x);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::multiplication_table;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1, 2, 3], [2, 4, 6], [3, 6, 9]]);
    }
}
