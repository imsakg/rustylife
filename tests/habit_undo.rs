use rustylife::habit::Habit;

#[test]
fn undo_is_idempotent_and_only_removes_the_selected_day() {
    let date = "2021-09-05".parse().unwrap();
    let tomorrow = "2021-09-06".parse().unwrap();
    let mut h = Habit::new(1, "Walk", date).unwrap();
    h.check_in(date).unwrap();
    h.check_in(tomorrow).unwrap();
    assert!(h.undo(date));
    assert!(!h.undo(date));
    assert!(h.checkins().contains(&tomorrow));
}
