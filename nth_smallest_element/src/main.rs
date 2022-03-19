fn main() {
    println!("{:?}", nth_smallest(&[15, 20, 7, 10, 4, 3], 3));
}

fn nth_smallest(nums: &[i32], pos: usize) -> i32 {
    let mut mut_nums: Vec<i32> = Vec::from(nums);

    mut_nums.sort();
    mut_nums.to_vec()[pos - 1]
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::nth_smallest;

    #[test]
    fn sample_tests() {
        assert_eq!(nth_smallest(&[3, 1, 2], 2), 2);
        assert_eq!(nth_smallest(&[15, 20, 7, 10, 4, 3], 3), 7);
        assert_eq!(nth_smallest(&[-5, -1, -6, -18], 4), -1);
        assert_eq!(nth_smallest(&[-102, -16, -1, -2, -367, -9], 5), -2);
        assert_eq!(nth_smallest(&[2, 169, 13, -5, 0, -1], 4), 2);
    }
}
