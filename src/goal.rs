use crate::{date::Date, habit::valid_name, Result};
use serde::{Deserialize, Serialize};

/// A measurable goal using whole units; progress is always within its target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawGoal")]
pub struct Goal {
    id: u64,
    name: String,
    target: u64,
    progress: u64,
    unit: String,
    created: Date,
    due: Option<Date>,
    milestones: Vec<Milestone>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawMilestone")]
pub struct Milestone {
    name: String,
    done: bool,
}
impl Milestone {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn done(&self) -> bool {
        self.done
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawGoal {
    id: u64,
    name: String,
    target: u64,
    progress: u64,
    unit: String,
    created: Date,
    due: Option<Date>,
    milestones: Vec<Milestone>,
}
impl TryFrom<RawGoal> for Goal {
    type Error = String;
    fn try_from(raw: RawGoal) -> Result<Self> {
        let goal = Self {
            id: raw.id,
            name: raw.name,
            target: raw.target,
            progress: raw.progress,
            unit: raw.unit,
            created: raw.created,
            due: raw.due,
            milestones: raw.milestones,
        };
        goal.validate()?;
        Ok(goal)
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMilestone {
    name: String,
    done: bool,
}
impl TryFrom<RawMilestone> for Milestone {
    type Error = String;
    fn try_from(raw: RawMilestone) -> Result<Self> {
        if valid_name(&raw.name)? != raw.name {
            return Err("invalid milestone name".into());
        }
        Ok(Self {
            name: raw.name,
            done: raw.done,
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
            milestones: Vec::new(),
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

    /// Add a named step and return its one-based position.
    pub fn add_milestone(&mut self, name: &str) -> Result<usize> {
        let name = valid_name(name)?;
        if self.milestones.len() >= 100 || self.milestones.iter().any(|m| m.name == name) {
            return Err("milestone is duplicated or the 100-step limit was reached".into());
        }
        self.milestones.push(Milestone { name, done: false });
        Ok(self.milestones.len())
    }
    pub fn set_milestone(&mut self, number: usize, done: bool) -> Result<()> {
        let index = number
            .checked_sub(1)
            .ok_or("milestone numbers start at 1")?;
        self.milestones
            .get_mut(index)
            .ok_or("milestone not found")?
            .done = done;
        Ok(())
    }
    pub fn milestones(&self) -> &[Milestone] {
        &self.milestones
    }

    pub(crate) fn validate(&self) -> Result<()> {
        if self.id == 0
            || self.target == 0
            || self.progress > self.target
            || valid_name(&self.name)? != self.name
            || valid_name(&self.unit)? != self.unit
        {
            return Err("invalid goal identity, target, progress or name".into());
        }
        if self.due.is_some_and(|d| d < self.created) || self.milestones.len() > 100 {
            return Err("invalid goal deadline or milestone count".into());
        }
        let mut names = std::collections::BTreeSet::new();
        for m in &self.milestones {
            if valid_name(&m.name)? != m.name || !names.insert(&m.name) {
                return Err("invalid or duplicated milestone".into());
            }
        }
        Ok(())
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
