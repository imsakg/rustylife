use crate::{date::Date, store::Store, Result};
use std::path::PathBuf;

pub const HELP: &str = "rustylife — local habits and goals
Usage: rustylife [--data PATH] [--on YYYY-MM-DD] COMMAND

habit add NAME                  Create a daily habit
habit list [all]                List active habits, or include archived habits
habit check ID                 Record the selected day (idempotent)
habit undo ID                  Remove a mistaken check-in
habit rename ID NAME           Rename without losing history
habit archive ID | resume ID   Archive or resume a habit

Options must precede the command; quote names containing spaces.
Data: --data, then RUSTYLIFE_DATA, then $XDG_DATA_HOME/rustylife/data.json
(or $HOME/.local/share/rustylife/data.json). --on defaults to local today.
";

pub fn run(args: &[String]) -> Result<String> {
    let mut rest = args;
    let mut path = None;
    let mut today = None;
    loop {
        match rest.first().map(String::as_str) {
            Some("--data") => {
                let value = rest.get(1).ok_or("--data requires a path")?;
                if value.is_empty() {
                    return Err("data path must not be empty".into());
                }
                path = Some(PathBuf::from(value));
                rest = &rest[2..];
            }
            Some("--on") => {
                today = Some(
                    rest.get(1)
                        .ok_or("--on requires YYYY-MM-DD")?
                        .parse::<Date>()?,
                );
                rest = &rest[2..];
            }
            _ => break,
        }
    }
    let words: Vec<&str> = rest.iter().map(String::as_str).collect();
    if matches!(words.as_slice(), [] | ["--help"] | ["help"] | ["-h"]) {
        return Ok(HELP.into());
    }
    if words.as_slice() == ["--version"] {
        return Ok(format!("rustylife {}\n", env!("CARGO_PKG_VERSION")));
    }
    let path = match path {
        Some(path) => path,
        None => default_path()?,
    };
    let store = Store::open(path)?;
    execute(&store, &words, today.unwrap_or_else(Date::today))
}
fn default_path() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("RUSTYLIFE_DATA") {
        if path.is_empty() {
            return Err("RUSTYLIFE_DATA must not be empty".into());
        }
        return Ok(path.into());
    }
    if let Some(base) = std::env::var_os("XDG_DATA_HOME").filter(|v| !v.is_empty()) {
        return Ok(PathBuf::from(base).join("rustylife/data.json"));
    }
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .ok_or("no home directory; pass --data PATH")?;
    Ok(PathBuf::from(home).join(".local/share/rustylife/data.json"))
}
fn id(value: &str) -> Result<u64> {
    let id: u64 = value
        .parse()
        .map_err(|_| "expected a positive integer id")?;
    if id == 0 {
        return Err("id must be positive".into());
    }
    Ok(id)
}
fn execute(store: &Store, words: &[&str], today: Date) -> Result<String> {
    match words {
        ["habit", "list"] | ["habit", "list", "all"] => {
            let state = store.read()?;
            let all = words.len() == 3;
            let mut lines = String::new();
            for h in state.habits().iter().filter(|h| all || !h.is_archived()) {
                lines.push_str(&format!(
                    "{}: {} [{}] — {} check-ins\n",
                    h.id(),
                    h.name(),
                    if h.is_archived() {
                        "archived"
                    } else {
                        "active"
                    },
                    h.checkins().len()
                ));
            }
            if lines.is_empty() {
                lines.push_str("No habits. Add one with: habit add NAME\n");
            }
            Ok(lines)
        }
        ["habit", "add", name] => store.update(|s| {
            Ok(format!(
                "Created habit {}: {name}\n",
                s.add_habit(name, today)?
            ))
        }),
        ["habit", "check", value] => {
            let id = id(value)?;
            store.update(|s| {
                let added = s.habit_mut(id)?.check_in(today)?;
                Ok(format!(
                    "{} habit {id} on {today}\n",
                    if added {
                        "Recorded"
                    } else {
                        "Already recorded"
                    }
                ))
            })
        }
        ["habit", "undo", value] => {
            let id = id(value)?;
            store.update(|s| {
                Ok(format!(
                    "{} check-in for habit {id} on {today}\n",
                    if s.habit_mut(id)?.undo(today) {
                        "Removed"
                    } else {
                        "No"
                    }
                ))
            })
        }
        ["habit", "rename", value, name] => {
            let id = id(value)?;
            store.update(|s| {
                s.habit_mut(id)?.rename(name)?;
                Ok(format!("Renamed habit {id}\n"))
            })
        }
        ["habit", action @ ("archive" | "resume"), value] => {
            let id = id(value)?;
            store.update(|s| {
                s.habit_mut(id)?.set_archived(*action == "archive");
                Ok(format!("Updated habit {id}\n"))
            })
        }
        _ => Err("unknown command or wrong arguments; run rustylife --help".into()),
    }
}
