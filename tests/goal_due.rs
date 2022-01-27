use rustylife::{
    date::Date,
    goal::{Goal, GoalStatus},
};
fn d(s: &str) -> Date {
    s.parse().unwrap()
}
#[test]
fn deadlines_become_overdue_only_after_the_due_day() {
    let mut g = Goal::new(1, "Read", 2, "books", d("2021-09-01")).unwrap();
    assert_eq!(g.status(d("2021-09-03")), GoalStatus::Active);
    assert!(g.set_due(Some(d("2021-08-31"))).is_err());
    g.set_due(Some(d("2021-09-02"))).unwrap();
    assert_eq!(g.due(), Some(d("2021-09-02")));
    assert_eq!(g.status(d("2021-09-02")), GoalStatus::Active);
    assert_eq!(g.status(d("2021-09-03")), GoalStatus::Overdue);
    g.advance(2).unwrap();
    assert_eq!(g.status(d("2021-09-03")), GoalStatus::Complete);
    g.set_due(None).unwrap();
    assert_eq!(g.due(), None);
}
