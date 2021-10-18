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

    pub fn checkins(&self) -> &BTreeSet<Date> {
        &self.checkins
    }
}
