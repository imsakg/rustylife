use rustylife::{date::Date, habit::Habit};
fn day(s: &str) -> Date {
    s.parse().unwrap()
}

#[test]
fn checkins_are_idempotent_and_cannot_predate_creation() {
    let mut h = Habit::new(1, "Walk", day("2021-09-05")).unwrap();
    assert!(h.check_in(day("2021-09-04")).is_err());
    assert!(h.check_in(day("2021-09-05")).unwrap());
    assert!(!h.check_in(day("2021-09-05")).unwrap());
    assert_eq!(h.checkins().len(), 1);
}
