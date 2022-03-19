fn main() {
    println!("{}", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

fn high_and_low(numbers: &str) -> String {
    let result: Vec<i32> = numbers
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let x = result.iter().min().unwrap();
    let y = result.iter().max().unwrap();
    format!("{} {}", y, x)
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}
