fn main() {
    println!("{:?}", count_sheep(&[true, false]));
}

fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&&sheep| sheep).count() as u8
}

#[test]
fn returns_correct_sheep_count() {
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
}
