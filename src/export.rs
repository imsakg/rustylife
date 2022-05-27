use crate::{date::Date, store::State};

/// RFC-4180 escaping plus a leading apostrophe for spreadsheet formula-like text.
/// Exports are for analysis; backups retain the exact original strings.
pub fn csv_field(value: &str) -> String {
    let safe = if value.trim_start().starts_with(['=', '+', '-', '@']) {
        format!("'{value}")
    } else {
        value.to_owned()
    };
    if safe.contains([',', '"', '\r', '\n']) {
        format!("\"{}\"", safe.replace('"', "\"\""))
    } else {
        safe
    }
}
pub fn habit_csv(state: &State) -> String {
    let mut text = "habit_id,habit_name,date,archived\r\n".to_owned();
    for h in state.habits() {
        for day in h.checkins() {
            text.push_str(&format!(
                "{},{},{},{}\r\n",
                h.id(),
                csv_field(h.name()),
                day,
                h.is_archived()
            ));
        }
    }
    text
}
pub fn goal_csv(state: &State, today: Date) -> String {
    let mut text =
        "goal_id,goal_name,progress,target,unit,due,status,milestones_done,milestones_total\r\n"
            .to_owned();
    for g in state.goals() {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\r\n",
            g.id(),
            csv_field(g.name()),
            g.progress(),
            g.target(),
            csv_field(g.unit()),
            g.due().map(|d| d.to_string()).unwrap_or_default(),
            g.status(today),
            g.milestones().iter().filter(|m| m.done()).count(),
            g.milestones().len()
        ));
    }
    text
}
