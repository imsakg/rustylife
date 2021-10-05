use rustylife::{date::Date, habit::Habit};
fn day(s: &str) -> Date {
    s.parse().unwrap()
}

#[test]
fn habits_require_nonblank_names_and_stable_positive_ids() {
    let habit = Habit::new(1, "  Read  ", day("2021-09-01")).unwrap();
    assert_eq!(habit.id(), 1);
    assert_eq!(habit.name(), "Read");
    assert_eq!(habit.created(), day("2021-09-01"));
    assert!(!habit.is_archived());
    assert!(Habit::new(0, "Read", day("2021-09-01")).is_err());
    assert!(Habit::new(1, " ", day("2021-09-01")).is_err());
    assert!(Habit::new(1, "bad\nname", day("2021-09-01")).is_err());
}
