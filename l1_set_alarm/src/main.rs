fn main() {
    println!("{:?}", set_alarm(true, true));
}
/// Write a function named set_alarm which receives two parameters. The first parameter, employed, is true whenever you are employed and the second parameter, vacation is true whenever you are on vacation.
///
/// The function should return true if you are employed and not on vacation (because these are the circumstances under which you need to set an alarm). It should return false otherwise. Examples:
///
/// set_alarm(true, true) -> false
/// set_alarm(false, true) -> false
/// set_alarm(false, false) -> false
/// set_alarm(true, false) -> true
fn set_alarm(employed: bool, vacation: bool) -> bool {
    matches!((employed, vacation), (true, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_alarm() {
        assert_eq!(
            set_alarm(true, true),
            false,
            "Fails when input is true, true"
        );
        assert_eq!(
            set_alarm(false, true),
            false,
            "Fails when input is false, true"
        );
        assert_eq!(
            set_alarm(false, false),
            false,
            "Fails when input is false, false"
        );
        assert_eq!(
            set_alarm(true, false),
            true,
            "Fails when input is true, false"
        );
    }
}
