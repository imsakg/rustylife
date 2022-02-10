use rustylife::store::State;
#[test]
fn snapshots_roundtrip_and_allocate_stable_ids() {
    let mut state = State::default();
    let date = "2021-09-01".parse().unwrap();
    assert_eq!(state.add_habit("Read", date).unwrap(), 1);
    assert_eq!(state.add_habit("Walk", date).unwrap(), 2);
    assert_eq!(state.add_goal("Books", 12, "books", date).unwrap(), 1);
    state.habit_mut(1).unwrap().check_in(date).unwrap();
    state.goal_mut(1).unwrap().advance(3).unwrap();
    let text = state.to_json().unwrap();
    assert_eq!(state, State::from_json(&text).unwrap());
    assert!(state.habit(99).is_err());
    assert!(state.goal(99).is_err());
}
#[test]
fn invalid_or_future_snapshots_are_not_silently_reset() {
    let mut state = State::default();
    state
        .add_habit("Read", "2021-09-01".parse().unwrap())
        .unwrap();
    let value: serde_json::Value = serde_json::from_str(&state.to_json().unwrap()).unwrap();
    let mut broken = value.clone();
    broken["schema_version"] = 2.into();
    assert!(State::from_json(&broken.to_string()).is_err());
    let mut broken = value.clone();
    broken["habits"][0]["name"] = " ".into();
    assert!(State::from_json(&broken.to_string()).is_err());
    let mut broken = value.clone();
    broken["habits"][0]["checkins"] = serde_json::json!(["2021-08-31"]);
    assert!(State::from_json(&broken.to_string()).is_err());
    let mut broken = value;
    broken["next_habit_id"] = 1.into();
    assert!(State::from_json(&broken.to_string()).is_err());
    assert!(State::from_json("broken").is_err());
}
