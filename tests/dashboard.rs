use rustylife::{report::dashboard, store::State};
#[test]
fn daily_dashboard_omits_archived_and_not_yet_created_items() {
    let mut state = State::default();
    let today = "2021-09-05".parse().unwrap();
    state.add_habit("Read", today).unwrap();
    state.habit_mut(1).unwrap().check_in(today).unwrap();
    state.add_habit("Old", today).unwrap();
    state.habit_mut(2).unwrap().set_archived(true);
    state
        .add_habit("Future", "2021-09-06".parse().unwrap())
        .unwrap();
    state.add_goal("Books", 4, "books", today).unwrap();
    state.goal_mut(1).unwrap().advance(1).unwrap();
    let text = dashboard(&state, today);
    assert!(text.contains("1/1 habits completed"));
    assert!(text.contains("[x] 1: Read — streak 1"));
    assert!(text.contains("[#####---------------] 25%"));
    assert!(!text.contains("Old"));
    assert!(!text.contains("Future"));
}
#[test]
fn empty_dashboard_suggests_a_small_first_action() {
    let text = dashboard(&State::default(), "0001-01-01".parse().unwrap());
    assert!(text.contains("habit add"));
    assert!(text.contains("0/0 habits completed"));
}
