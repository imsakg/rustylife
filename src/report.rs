use crate::{date::Date, store::State, visual::progress_bar, Result};

/// Rolling seven-day review through the reference day (inclusive).
pub fn weekly(state: &State, through: Date) -> Result<String> {
    let first = "0001-01-01".parse::<Date>()?;
    window(
        state,
        through.add_days(-6).unwrap_or(first),
        through,
        "Weekly review",
    )
}

/// Calendar month-to-date. Select the month's last day for a complete monthly review.
pub fn monthly(state: &State, through: Date) -> Result<String> {
    let start = format!("{:04}-{:02}-01", through.year(), through.month()).parse::<Date>()?;
    window(state, start, through, "Monthly review")
}

fn window(state: &State, start: Date, end: Date, title: &str) -> Result<String> {
    let mut text =
        format!("{title} — {start} .. {end}\nHabit completion (current active habits)\n");
    let (mut done, mut eligible) = (0_u64, 0_u64);
    for h in state
        .habits()
        .iter()
        .filter(|h| !h.is_archived() && h.created() <= end)
    {
        let c = h.completion(start, end)?;
        done += c.completed as u64;
        eligible += c.eligible as u64;
        text.push_str(&format!(
            "{}: {}/{} ({:.1}%) {}\n",
            h.name(),
            c.completed,
            c.eligible,
            c.percent(),
            progress_bar(c.completed as u64, c.eligible as u64, 14)
        ));
    }
    let percent = if eligible == 0 {
        0.0
    } else {
        done as f64 * 100.0 / eligible as f64
    };
    text.push_str(&format!(
        "Total: {done}/{eligible} ({percent:.1}%)\nGoals (current totals, not historical)\n"
    ));
    for g in state.goals().iter().filter(|g| g.created() <= end) {
        text.push_str(&format!(
            "{}: {}/{} {} [{}]\n",
            g.name(),
            g.progress(),
            g.target(),
            g.unit(),
            g.status(end)
        ));
    }
    Ok(text)
}

/// Render a deterministic overview. Reference dates do not rewind goal totals.
pub fn dashboard(state: &State, today: Date) -> String {
    let habits: Vec<_> = state
        .habits()
        .iter()
        .filter(|h| !h.is_archived() && h.created() <= today)
        .collect();
    let completed = habits
        .iter()
        .filter(|h| h.checkins().contains(&today))
        .count();
    let mut text = format!(
        "Today — {today}\n{completed}/{} habits completed\n",
        habits.len()
    );
    for h in &habits {
        text.push_str(&format!(
            "[{}] {}: {} — streak {} (best {})\n",
            if h.checkins().contains(&today) {
                "x"
            } else {
                " "
            },
            h.id(),
            h.name(),
            h.current_streak(today),
            h.longest_streak(today)
        ));
    }
    if habits.is_empty() {
        text.push_str("Start small: habit add \"Read for ten minutes\"\n");
    }
    text.push_str("Goals (current progress)\n");
    for g in state.goals().iter().filter(|g| g.created() <= today) {
        text.push_str(&format!(
            "{}: {} {} [{}]\n",
            g.id(),
            g.name(),
            progress_bar(g.progress(), g.target(), 20),
            g.status(today)
        ));
    }
    text
}
