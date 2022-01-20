use rustylife::goal::Goal;
#[test]
fn progress_can_be_added_or_corrected_without_overflow() {
    let mut g = Goal::new(1, "Read", 12, "books", "2021-09-01".parse().unwrap()).unwrap();
    g.advance(3).unwrap();
    assert_eq!(g.progress(), 3);
    assert_eq!(g.percent(), 25.0);
    g.set_progress(2).unwrap();
    assert_eq!(g.progress(), 2);
    assert!(g.advance(u64::MAX).is_err());
    assert_eq!(g.progress(), 2);
    assert!(g.set_progress(13).is_err());
    assert_eq!(g.progress(), 2);
    g.set_progress(12).unwrap();
    assert_eq!(g.percent(), 100.0);
}
