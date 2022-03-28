fn main() {
    println!("{:?}", tank_vol(5, 7, 3848));
}

fn tank_vol(h: i32, d: i32, vt: i32) -> i32 {
    // your code
    let radius = d as f64 / 2_f64;
    let l = vt as f64 / (std::f64::consts::PI * radius.powf(2_f64));

    //Vr = r2 × arccos (1 - h / r) - (r - h) × √(2 × r × h - h2)
    (l * (radius.powf(2_f64) * (1_f64 - h as f64 / radius).acos()
        - (radius - h as f64) * (2_f64 * radius * h as f64 - (h as f64).powf(2_f64)).sqrt()))
    .floor() as i32
}

fn dotest(h: i32, d: i32, vt: i32, exp: i32) {
    assert_eq!(tank_vol(h, d, vt), exp)
}
#[test]
fn basics_tank_vol() {
    dotest(5, 7, 3848, 2940);
    dotest(5, 7, 3848, 2940);
    dotest(2, 7, 3848, 907);
    dotest(2, 8, 5026, 982);
    dotest(4, 9, 6361, 2731);
}
