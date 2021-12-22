use crate::{date::Date, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// A daily habit. Check-ins are unique dates, not an incrementing counter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Habit {
    id: u64,
    name: String,
    created: Date,
    archived: bool,
    checkins: BTreeSet<Date>,
}

pub(crate) fn valid_name(name: &str) -> Result<String> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 120 || name.chars().any(char::is_control) {
        return Err("name must contain 1–120 printable characters".into());
    }
    Ok(name.to_owned())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Completion {
    pub completed: usize,
    pub eligible: usize,
}
impl Completion {
    pub fn percent(self) -> f64 {
        if self.eligible == 0 {
            0.0
        } else {
            self.completed as f64 * 100.0 / self.eligible as f64
        }
    }
}

impl Habit {
    pub fn new(id: u64, name: &str, created: Date) -> Result<Self> {
        if id == 0 {
            return Err("id must be positive".into());
        }
        Ok(Self {
            id,
            name: valid_name(name)?,
            created,
            archived: false,
            checkins: BTreeSet::new(),
        })
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn created(&self) -> Date {
        self.created
    }
    pub fn is_archived(&self) -> bool {
        self.archived
    }
    /// Returns false for an already-recorded day; duplicates never inflate progress.
    pub fn check_in(&mut self, date: Date) -> Result<bool> {
        if self.archived {
            return Err("cannot check in an archived habit".into());
        }
        if date < self.created {
            return Err("check-in cannot predate habit creation".into());
        }
        Ok(self.checkins.insert(date))
    }

    /// Correct a check-in, including historical records of archived habits.
    pub fn undo(&mut self, date: Date) -> bool {
        self.checkins.remove(&date)
    }

    pub fn rename(&mut self, name: &str) -> Result<()> {
        self.name = valid_name(name)?;
        Ok(())
    }
    pub fn set_archived(&mut self, archived: bool) {
        self.archived = archived;
    }

    /// Consecutive completed days ending today, or yesterday if today is unfinished.
    pub fn current_streak(&self, today: Date) -> usize {
        let anchor = if self.checkins.contains(&today) {
            Some(today)
        } else {
            today.add_days(-1).ok()
        };
        let Some(mut cursor) = anchor else {
            return 0;
        };
        let mut streak = 0;
        while self.checkins.contains(&cursor) {
            streak += 1;
            match cursor.add_days(-1) {
                Ok(previous) => cursor = previous,
                Err(_) => break,
            }
        }
        streak
    }

    pub fn longest_streak(&self, through: Date) -> usize {
        let mut previous: Option<Date> = None;
        let (mut current, mut longest) = (0, 0);
        for &date in self.checkins.range(..=through) {
            current = if previous.is_some_and(|p| p.days_until(date) == 1) {
                current + 1
            } else {
                1
            };
            longest = longest.max(current);
            previous = Some(date);
        }
        longest
    }

    pub fn completion(&self, start: Date, end: Date) -> Result<Completion> {
        if start > end {
            return Err("start date must not follow end date".into());
        }
        let start = start.max(self.created);
        if start > end {
            return Ok(Completion {
                completed: 0,
                eligible: 0,
            });
        }
        Ok(Completion {
            completed: self.checkins.range(start..=end).count(),
            eligible: (start.days_until(end) + 1) as usize,
        })
    }

    pub fn checkins(&self) -> &BTreeSet<Date> {
        &self.checkins
    }
}
