use rustylife::visual::progress_bar;
#[test]
fn bars_are_bounded_and_safe_for_zero_totals_and_extreme_values() {
    assert_eq!(progress_bar(1, 4, 8), "[##------] 25%");
    assert_eq!(progress_bar(0, 0, 5), "[-----] 0%");
    assert_eq!(progress_bar(8, 4, 5), "[#####] 100%");
    assert_eq!(progress_bar(u64::MAX, u64::MAX, 4), "[####] 100%");
    assert_eq!(progress_bar(1, 2, 0), "[] 50%");
    assert!(progress_bar(1, 2, usize::MAX).len() < 110);
}
