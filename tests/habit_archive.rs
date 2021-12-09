use rustylife::habit::Habit;

#[test]
fn archived_habits_keep_history_and_can_be_resumed() {
    let date = "2021-09-05".parse().unwrap();
    let mut h = Habit::new(1, "Walk", date).unwrap();
    h.check_in(date).unwrap();
    h.rename("  Evening walk  ").unwrap();
    assert_eq!(h.name(), "Evening walk");
    assert!(h.rename(" ").is_err());
    assert_eq!(h.name(), "Evening walk");
    h.set_archived(true);
    assert!(h.is_archived());
    assert!(h.check_in(date).is_err());
    assert_eq!(h.checkins().len(), 1);
    h.set_archived(false);
    assert!(!h.check_in(date).unwrap());
}
