use crate::{date::Date, habit::valid_name, Result};
use serde::{Deserialize, Serialize};

/// A measurable goal using whole units; progress is always within its target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    id: u64,
    name: String,
    target: u64,
    progress: u64,
    unit: String,
    created: Date,
    due: Option<Date>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoalStatus {
    Active,
    Overdue,
    Complete,
}
impl std::fmt::Display for GoalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Active => "active",
            Self::Overdue => "overdue",
            Self::Complete => "complete",
        })
    }
}

impl Goal {
    pub fn new(id: u64, name: &str, target: u64, unit: &str, created: Date) -> Result<Self> {
        if id == 0 || target == 0 {
            return Err("id and target must be positive".into());
        }
        Ok(Self {
            id,
            name: valid_name(name)?,
            target,
            progress: 0,
            due: None,
            unit: valid_name(unit)?,
            created,
        })
    }
    pub fn set_progress(&mut self, progress: u64) -> Result<()> {
        if progress > self.target {
            return Err("progress cannot exceed target".into());
        }
        self.progress = progress;
        Ok(())
    }
    pub fn advance(&mut self, amount: u64) -> Result<()> {
        self.set_progress(
            self.progress
                .checked_add(amount)
                .ok_or("progress overflow")?,
        )
    }

    pub fn due(&self) -> Option<Date> {
        self.due
    }
    pub fn set_due(&mut self, due: Option<Date>) -> Result<()> {
        if due.is_some_and(|d| d < self.created) {
            return Err("due date cannot predate goal creation".into());
        }
        self.due = due;
        Ok(())
    }
    pub fn status(&self, today: Date) -> GoalStatus {
        if self.progress == self.target {
            GoalStatus::Complete
        } else if self.due.is_some_and(|d| d < today) {
            GoalStatus::Overdue
        } else {
            GoalStatus::Active
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn target(&self) -> u64 {
        self.target
    }
    pub fn progress(&self) -> u64 {
        self.progress
    }
    pub fn unit(&self) -> &str {
        &self.unit
    }
    pub fn created(&self) -> Date {
        self.created
    }
    pub fn percent(&self) -> f64 {
        self.progress as f64 * 100.0 / self.target as f64
    }
}
