fn main() {
    println!("{}", round_to_next_5(-16));
}

/// Given an integer as input, can you round it to the next (meaning, "higher") multiple of 5?
///
/// Examples:
///
/// input:    output:
/// 0    ->   0
/// 2    ->   5
/// 3    ->   5
/// 12   ->   15
/// 21   ->   25
/// 30   ->   30
/// -2   ->   0
/// -5   ->   -5
/// etc.
///
/// Input may be any positive or negative integer (including 0).
///
/// You can assume that all inputs are valid integers.
fn round_to_next_5(n: i32) -> i32 {
    match n {
        0 => 0,
        x if x % 5 == 0 => x,
        x if x < 0 => n - n % 5,
        x if x > 0 => n + (5 - n % 5),
        _ => unreachable!(),
    }
    // n + (5 - n % 5) % 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(round_to_next_5(1), 5);
    }

    #[test]
    fn test_basic_neg() {
        assert_eq!(round_to_next_5(-1), 0);
    }

    #[test]
    fn test_extended() {
        let tests = [
            [0, 0],
            [1, 5],
            [-1, 0],
            [-5, -5],
            [3, 5],
            [5, 5],
            [7, 10],
            [20, 20],
            [39, 40],
            [990, 990],
            [121, 125],
            [555, 555],
        ];

        for [x, out] in tests.iter() {
            assert_eq!(round_to_next_5(*x), *out);
        }
    }
}
