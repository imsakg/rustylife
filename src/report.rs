use crate::{date::Date, store::State, visual::progress_bar};

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
