fn main() {
    println!("{:?}", dig_pow(46288, 3));
}

/// Some numbers have funny properties. For example:
///
/// 89 --> 8¹ + 9² = 89 * 1
///
/// 695 --> 6² + 9³ + 5⁴= 1390 = 695 * 2
///
/// 46288 --> 4³ + 6⁴+ 2⁵ + 8⁶ + 8⁷ = 2360688 = 46288 * 51
///
/// Given a positive integer n written as abcd... (a, b, c, d... being digits) and a positive integer p
///
/// we want to find a positive integer k, if it exists, such that the sum of the digits of n taken to the successive powers of p is equal to k * n.
///
/// In other words:
///
/// Is there an integer k such as : (a ^ p + b ^ (p+1) + c ^(p+2) + d ^ (p+3) + ...) = n * k
///
/// If it is the case we will return k, if not return -1.
///
/// Note: n and p will always be given as strictly positive integers.
///
/// dig_pow(89, 1) should return 1 since 8¹ + 9² = 89 = 89 * 1
/// dig_pow(92, 1) should return -1 since there is no k such as 9¹ + 2² equals 92 * k
/// dig_pow(695, 2) should return 2 since 6² + 9³ + 5⁴= 1390 = 695 * 2
/// dig_pow(46288, 3) should return 51 since 4³ + 6⁴+ 2⁵ + 8⁶ + 8⁷ = 2360688 = 46288 * 51
fn dig_pow(n: i64, p: i32) -> i64 {
    let p = p as u32;

    let digits = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<_>>();
    let result = digits
        .iter()
        .enumerate()
        .fold(0, |nk, (i, &digit)| nk + digit.pow(p + i as u32));

    match result % n {
        0 => result / n,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(89, 1, 1);
        dotest(92, 1, -1);
        dotest(46288, 3, 51);
    }
}
