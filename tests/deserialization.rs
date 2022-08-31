use rustylife::{
    goal::{Goal, Milestone},
    habit::Habit,
    store::State,
};

#[test]
fn direct_goal_deserialization_cannot_bypass_invariants() {
    let g = Goal::new(1, "Read", 12, "books", "2021-09-01".parse().unwrap()).unwrap();
    let value = serde_json::to_value(&g).unwrap();
    for (key, replacement) in [
        ("id", serde_json::json!(0)),
        ("target", serde_json::json!(0)),
        ("progress", serde_json::json!(13)),
        ("name", serde_json::json!(" ")),
        ("due", serde_json::json!("2021-08-31")),
    ] {
        let mut bad = value.clone();
        bad[key] = replacement;
        assert!(serde_json::from_value::<Goal>(bad).is_err(), "{key}");
    }
    let restored: Goal = serde_json::from_value(value).unwrap();
    assert_eq!(restored, g);
}
#[test]
fn direct_habit_deserialization_rejects_invalid_names_dates_and_duplicate_entries() {
    let h = Habit::new(1, "Read", "2021-09-01".parse().unwrap()).unwrap();
    let value = serde_json::to_value(&h).unwrap();
    for (key, replacement) in [
        ("id", serde_json::json!(0)),
        ("name", serde_json::json!(" Read ")),
        ("checkins", serde_json::json!(["2021-08-31"])),
        ("checkins", serde_json::json!(["2021-09-01", "2021-09-01"])),
        ("category", serde_json::json!("bad\ncategory")),
    ] {
        let mut bad = value.clone();
        bad[key] = replacement;
        assert!(serde_json::from_value::<Habit>(bad).is_err(), "{key}");
    }
    let mut unknown = value.clone();
    unknown["typo"] = true.into();
    assert!(serde_json::from_value::<Habit>(unknown).is_err());
    assert_eq!(serde_json::from_value::<Habit>(value).unwrap(), h);
}
#[test]
fn direct_state_deserialization_checks_version_and_id_collisions() {
    let mut s = State::default();
    s.add_habit("Read", "2021-09-01".parse().unwrap()).unwrap();
    let value = serde_json::to_value(&s).unwrap();
    let mut bad = value.clone();
    bad["schema_version"] = 999.into();
    assert!(serde_json::from_value::<State>(bad).is_err());
    let mut bad = value.clone();
    bad["next_habit_id"] = 0.into();
    assert!(serde_json::from_value::<State>(bad).is_err());
    let mut bad = value.clone();
    bad["habits"]
        .as_array_mut()
        .unwrap()
        .push(value["habits"][0].clone());
    assert!(serde_json::from_value::<State>(bad).is_err());
    assert_eq!(serde_json::from_value::<State>(value).unwrap(), s);
}
#[test]
fn standalone_milestones_must_also_be_valid() {
    assert!(serde_json::from_str::<Milestone>(r#"{"name":" ","done":false}"#).is_err());
    assert!(serde_json::from_str::<Milestone>(r#"{"name":"Step","done":true,"extra":1}"#).is_err());
    let m: Milestone = serde_json::from_str(r#"{"name":"Step","done":true}"#).unwrap();
    assert!(m.done());
    assert_eq!(m.name(), "Step");
}
