use rustylife::goal::Goal;
#[test]
fn measurable_goals_validate_target_name_and_units() {
    let date = "2021-09-01".parse().unwrap();
    let g = Goal::new(1, "Read books", 12, "books", date).unwrap();
    assert_eq!(
        (g.id(), g.name(), g.target(), g.progress(), g.unit()),
        (1, "Read books", 12, 0, "books")
    );
    assert_eq!(g.percent(), 0.0);
    assert!(Goal::new(1, "Read", 0, "books", date).is_err());
    assert!(Goal::new(0, "Read", 12, "books", date).is_err());
    assert!(Goal::new(1, " ", 12, "books", date).is_err());
    assert!(Goal::new(1, "Read", 12, "", date).is_err());
}
