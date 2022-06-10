use rustylife::{habit::Habit, store::State};
#[test]
fn categories_are_validated_optional_and_backward_compatible() {
    let day = "2021-09-01".parse().unwrap();
    let mut h = Habit::new(1, "Walk", day).unwrap();
    assert_eq!(h.category(), None);
    h.set_category(Some("  Health  ")).unwrap();
    assert_eq!(h.category(), Some("Health"));
    assert!(h.set_category(Some(" ")).is_err());
    assert_eq!(h.category(), Some("Health"));
    h.set_category(None).unwrap();
    assert_eq!(h.category(), None);
    let mut s = State::default();
    s.add_habit("Read", day).unwrap();
    let mut json: serde_json::Value = serde_json::from_str(&s.to_json().unwrap()).unwrap();
    json["habits"][0]
        .as_object_mut()
        .unwrap()
        .remove("category");
    assert!(State::from_json(&json.to_string()).is_ok());
}
#[test]
fn cli_category_filter_only_shows_matching_active_habits() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data.json");
    let run = |words: &[&str]| {
        let out = std::process::Command::new(env!("CARGO_BIN_EXE_rustylife"))
            .arg("--data")
            .arg(&path)
            .args(words)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    };
    run(&["habit", "add", "Walk"]);
    run(&["habit", "add", "Read"]);
    run(&["habit", "category", "1", "Health"]);
    let output = run(&["habit", "list", "category", "Health"]);
    assert!(output.contains("Walk"));
    assert!(!output.contains("Read"));
    run(&["habit", "category", "1", "none"]);
    assert!(!run(&["habit", "list", "category", "Health"]).contains("Walk"));
}
