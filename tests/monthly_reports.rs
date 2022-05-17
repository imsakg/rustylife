use rustylife::{report::monthly, store::State};
#[test]
fn monthly_review_handles_leap_years_and_month_to_date() {
    let mut state = State::default();
    state
        .add_habit("Read", "2020-02-01".parse().unwrap())
        .unwrap();
    state
        .habit_mut(1)
        .unwrap()
        .check_in("2020-02-29".parse().unwrap())
        .unwrap();
    let full = monthly(&state, "2020-02-29".parse().unwrap()).unwrap();
    assert!(full.contains("2020-02-01 .. 2020-02-29"));
    assert!(full.contains("Read: 1/29"));
    let partial = monthly(&state, "2020-02-10".parse().unwrap()).unwrap();
    assert!(partial.contains("Read: 0/10"));
    assert!(monthly(&State::default(), "9999-12-31".parse().unwrap()).is_ok());
}
