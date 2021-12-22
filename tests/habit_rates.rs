use rustylife::{date::Date, habit::Habit};
fn d(s: &str) -> Date {
    s.parse().unwrap()
}
#[test]
fn completion_counts_only_eligible_days_in_an_inclusive_window() {
    let mut h = Habit::new(1, "Run", d("2021-09-03")).unwrap();
    h.check_in(d("2021-09-03")).unwrap();
    h.check_in(d("2021-09-05")).unwrap();
    let c = h.completion(d("2021-09-01"), d("2021-09-06")).unwrap();
    assert_eq!((c.completed, c.eligible), (2, 4));
    assert_eq!(c.percent(), 50.0);
    let early = h.completion(d("2021-08-01"), d("2021-08-31")).unwrap();
    assert_eq!(early.percent(), 0.0);
    assert_eq!(early.eligible, 0);
    assert!(h.completion(d("2021-09-06"), d("2021-09-01")).is_err());
}
