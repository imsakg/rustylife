use crate::{date::Date, habit::Habit, Result};

/// Seven-day rows beginning at the selected start (not aligned to weekdays).
/// Limit output to 366 days so an accidental enormous window cannot flood a terminal.
pub fn habit_calendar(habit: &Habit, start: Date, end: Date) -> Result<String> {
    let days = start.days_until(end) + 1;
    if !(1..=366).contains(&days) {
        return Err("calendar range must contain 1–366 days".into());
    }
    let mut text = format!("{start} .. {end}\n");
    for offset in 0..days {
        let date = start.add_days(offset)?;
        text.push(if date < habit.created() {
            ' '
        } else if habit.checkins().contains(&date) {
            '#'
        } else {
            '.'
        });
        if (offset + 1) % 7 == 0 || offset + 1 == days {
            text.push('\n');
        }
    }
    text.push_str("# done  . missed  space before creation\n");
    Ok(text)
}

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
