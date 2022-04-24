fn main() {
    println!(
        "{}",
        find_average(&[
            17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,
        ])
    );
}

fn find_average(slice: &[f64]) -> f64 {
    match slice.len() {
        0 => 0.0,
        n => slice.iter().sum::<f64>() / n as f64,
    }
}
