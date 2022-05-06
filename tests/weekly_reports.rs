use rustylife::{report::weekly, store::State};
#[test]
fn weekly_review_uses_seven_days_and_weights_by_eligible_days() {
    let mut state = State::default();
    let end = "2021-09-07".parse().unwrap();
    state
        .add_habit("Read", "2021-09-01".parse().unwrap())
        .unwrap();
    state
        .add_habit("Walk", "2021-09-07".parse().unwrap())
        .unwrap();
    state
        .habit_mut(1)
        .unwrap()
        .check_in("2021-09-01".parse().unwrap())
        .unwrap();
    state.habit_mut(2).unwrap().check_in(end).unwrap();
    let text = weekly(&state, end).unwrap();
    assert!(text.contains("2021-09-01 .. 2021-09-07"));
    assert!(text.contains("Read: 1/7"));
    assert!(text.contains("Walk: 1/1"));
    assert!(text.contains("Total: 2/8 (25.0%)"));
    assert!(text.contains("not historical"));
}
#[test]
fn first_calendar_week_is_clamped_at_the_supported_minimum() {
    assert!(weekly(&State::default(), "0001-01-01".parse().unwrap()).is_ok());
}
