use rustylife::{date::Date, habit::Habit};
fn day(s: &str) -> Date {
    s.parse().unwrap()
}
#[test]
fn streaks_ignore_future_entries_and_allow_an_unfinished_today() {
    let mut h = Habit::new(1, "Read", day("2021-09-01")).unwrap();
    for date in [
        "2021-09-01",
        "2021-09-02",
        "2021-09-04",
        "2021-09-05",
        "2021-09-06",
        "2021-09-20",
    ] {
        h.check_in(day(date)).unwrap();
    }
    assert_eq!(h.current_streak(day("2021-09-06")), 3);
    assert_eq!(h.current_streak(day("2021-09-07")), 3);
    assert_eq!(h.current_streak(day("2021-09-08")), 0);
    assert_eq!(h.longest_streak(day("2021-09-03")), 2);
    assert_eq!(h.longest_streak(day("2021-09-07")), 3);
}
#[test]
fn empty_and_minimum_date_streaks_do_not_panic() {
    let date = day("0001-01-01");
    let mut h = Habit::new(1, "Read", date).unwrap();
    assert_eq!(h.current_streak(date), 0);
    assert_eq!(h.longest_streak(date), 0);
    h.check_in(date).unwrap();
    assert_eq!(h.current_streak(date), 1);
}
