fn main() {
    println!("{:?}", find_nb(4183059834009));
}

/// Your task is to construct a building which will be a pile of n cubes. The cube at the bottom will have a volume of n^3, the cube above will have volume of (n-1)^3 and so on until the top which will have a volume of 1^3.
///
/// You are given the total volume m of the building. Being given m can you find the number n of cubes you will have to build?
///
/// The parameter of the function findNb (find_nb, find-nb, findNb, ...) will be an integer m and you have to return the integer n such as n^3 + (n-1)^3 + ... + 1^3 = m if such a n exists or -1 if there is no such n.
/// Examples:
///
/// findNb(1071225) --> 45
///
/// findNb(91716553919377) --> -1
fn find_nb(m: u64) -> i32 {
    let mut n = 0;
    let mut mass = 0;

    while mass < m {
        n += 1;
        mass += (n as u64).pow(3);
    }

    if mass == m {
        n
    } else {
        -1
    }
}

fn testing(n: u64, exp: i32) {
    assert_eq!(find_nb(n), exp);
}

#[test]
fn basics_find_nb() {
    testing(4183059834009, 2022);
    testing(24723578342962, -1);
    testing(135440716410000, 4824);
    testing(40539911473216, 3568);
    testing(26825883955641, 3218);
}
