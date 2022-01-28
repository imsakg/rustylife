use rustylife::goal::Goal;
#[test]
fn milestones_are_named_unique_and_reversible() {
    let mut g = Goal::new(1, "Read", 2, "books", "2021-09-01".parse().unwrap()).unwrap();
    assert_eq!(g.add_milestone("  Choose books  ").unwrap(), 1);
    assert!(g.add_milestone("Choose books").is_err());
    assert!(g.add_milestone(" ").is_err());
    g.set_milestone(1, true).unwrap();
    assert!(g.milestones()[0].done());
    assert_eq!(g.milestones()[0].name(), "Choose books");
    assert!(g.set_milestone(0, true).is_err());
    assert!(g.set_milestone(2, true).is_err());
    g.set_milestone(1, false).unwrap();
    assert!(!g.milestones()[0].done());
    assert_eq!(g.progress(), 0);
}
