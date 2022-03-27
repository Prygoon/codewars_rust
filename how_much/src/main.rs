use std::mem::swap;

fn main() {
    println!("{:?}", how_much(1000, 1100));
}

fn how_much(mut m: i32, mut n: i32) -> Vec<(String, String, String)> {
    if m > n {
        swap(&mut m, &mut n);
    }

    ((m + 25) / 63..(n + 26) / 63)
        .map(|k| {
            (
                format!("{}: {}", 'M', k * 63 + 37),
                format!("{}: {}", 'B', k * 9 + 5),
                format!("{}: {}", 'C', k * 7 + 4),
            )
        })
        .collect()
}

fn testing(m: i32, n: i32, exp: Vec<(&str, &str, &str)>) {
    let ans: String = format!("{:?}", how_much(m, n));
    let sol: String = format!("{:?}", exp);
    assert_eq!(ans, sol)
}

#[test]
fn basics_how_much() {
    testing(
        1,
        100,
        vec![("M: 37", "B: 5", "C: 4"), ("M: 100", "B: 14", "C: 11")],
    );
    testing(1000, 1100, vec![("M: 1045", "B: 149", "C: 116")]);
    testing(10000, 9950, vec![("M: 9991", "B: 1427", "C: 1110")]);
    testing(
        0,
        200,
        vec![
            ("M: 37", "B: 5", "C: 4"),
            ("M: 100", "B: 14", "C: 11"),
            ("M: 163", "B: 23", "C: 18"),
        ],
    );
    testing(2950, 2950, vec![]);
}
