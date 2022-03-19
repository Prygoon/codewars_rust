fn main() {
    println!("{:?}", xo("Xo"));
}

fn xo(string: &'static str) -> bool {
    string.to_lowercase().matches("x").count() == string.to_lowercase().matches("o").count()
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
