/// An ASCII progress bar; width is capped at 80 and arithmetic cannot overflow.
pub fn progress_bar(completed: u64, total: u64, width: usize) -> String {
    let width = width.min(80);
    let (filled, percent) = if total == 0 {
        (0, 0)
    } else {
        let completed = completed.min(total) as u128;
        (
            (completed * width as u128 / total as u128) as usize,
            completed * 100 / total as u128,
        )
    };
    format!(
        "[{}{}] {}%",
        "#".repeat(filled),
        "-".repeat(width - filled),
        percent
    )
}
