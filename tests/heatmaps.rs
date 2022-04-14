use rustylife::{date::Date, habit::Habit, visual::habit_calendar};
fn d(s: &str) -> Date {
    s.parse().unwrap()
}
#[test]
fn calendar_marks_completed_missed_and_not_yet_created_days() {
    let mut h = Habit::new(1, "Read", d("2021-09-03")).unwrap();
    h.check_in(d("2021-09-04")).unwrap();
    assert_eq!(
        habit_calendar(&h, d("2021-09-01"), d("2021-09-05")).unwrap(),
        "2021-09-01 .. 2021-09-05\n  .#.\n# done  . missed  space before creation\n"
    );
    assert!(habit_calendar(&h, d("2021-09-05"), d("2021-09-01")).is_err());
    assert!(habit_calendar(&h, d("2021-01-01"), d("2022-01-02")).is_err());
}
#[test]
fn calendar_wraps_at_week_sized_chunks_without_losing_the_last_day() {
    let h = Habit::new(1, "Read", d("2021-09-01")).unwrap();
    let text = habit_calendar(&h, d("2021-09-01"), d("2021-09-08")).unwrap();
    assert!(text.contains(".......\n.\n"));
}
