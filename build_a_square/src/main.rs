fn main() {
    println!("{}", generate_shape(4));
}

fn generate_shape(n: i32) -> String {
    let str: String = "+".to_string().repeat(n as usize);
    let mut str_vec: Vec<String> = Vec::new();

    for _ in 0..n {
        str_vec.push(str.to_string());
    }

    str_vec.join("\n")

    // vec!["+".repeat(n as usize); n as usize].join("\n")
}

#[test]
fn sample_test() {
    assert_eq!(generate_shape(3), "+++\n+++\n+++");
}
