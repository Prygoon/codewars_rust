fn main() {
    println!(
        "{:?}",
        leaderboard_sort(
            &[
                "John".into(),
                "Brian".into(),
                "Jim".into(),
                "Dave".into(),
                "Fred".into()
            ],
            &["Dave +1".into(), "Fred +4".into(), "Brian -1".into()]
        )
    );
}

fn leaderboard_sort(leaderboard: &[String], changes: &[String]) -> Vec<String> {
    let mut leaderboard = leaderboard.to_vec();

    for change in changes {
        let (name, change) = change.split_once(' ').unwrap();
        let change: isize = change.parse().unwrap();

        let pos = leaderboard.iter().position(|s| s == name).unwrap();
        let name = leaderboard.remove(pos);
        leaderboard.insert((pos as isize - change) as _, name)
    }
    leaderboard
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            leaderboard_sort(
                &[
                    "John".into(),
                    "Brian".into(),
                    "Jim".into(),
                    "Dave".into(),
                    "Fred".into()
                ],
                &["Dave +1".into(), "Fred +4".into(), "Brian -1".into()],
            ),
            &["Fred", "John", "Dave", "Brian", "Jim"],
        );
        assert_eq!(
            leaderboard_sort(
                &[
                    "Bob".into(),
                    "Larry".into(),
                    "Kevin".into(),
                    "Jack".into(),
                    "Max".into()
                ],
                &["Max +3".into(), "Kevin -1".into(), "Kevin +3".into()],
            ),
            &["Bob", "Kevin", "Max", "Larry", "Jack"],
        );
    }
}
