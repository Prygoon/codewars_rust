fn main() {
    println!("{}", accum("ZpglnRxqenU"));
}

fn accum(s: &str) -> String {
    let mut str_vec: Vec<String> = Vec::new();

    for (i, char) in s.to_lowercase().chars().enumerate() {
        let string = char.to_string().repeat(i + 1);
        str_vec.push(string)
        }
    let result = str_vec.iter().map(|x| first_letter_to_upper_case(x)).collect::<Vec<String>>();

    result.join("-")

    // s.chars()
    //     .enumerate()
    //     .map(|(i, c)| {
    //         c.to_string().to_uppercase() + c.to_string().to_lowercase().repeat(i).as_str()
    //     })
    //     .collect::<Vec<String>>()
    //     .join("-")
}

fn first_letter_to_upper_case(s1: &String) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn basic_tests() {
    assert_eq!(
        accum("ZpglnRxqenU"),
        "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
    );
    assert_eq!(
        accum("NyffsGeyylB"),
        "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
    );
    assert_eq!(
        accum("MjtkuBovqrU"),
        "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
    );
    assert_eq!(
        accum("EvidjUnokmM"),
        "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
    );
    assert_eq!(
        accum("HbideVbxncC"),
        "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
    );
}
