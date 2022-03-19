fn main() {
    println!("{:?}", angle(10));
}

///Find the total sum of internal angles (in degrees) in an n-sided simple polygon. N will be greater than 2.
fn angle(n: u32) -> u32 {
    (n - 2) * 180
}

#[test]
fn normal_tests() {
    assert_eq!(angle(3), 180);
    assert_eq!(angle(4), 360);
    assert_eq!(angle(5), 540);
}
