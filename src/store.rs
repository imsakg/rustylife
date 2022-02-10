use crate::{date::Date, goal::Goal, habit::Habit, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const SCHEMA_VERSION: u32 = 1;

/// Portable, versioned application state. IDs remain stable across reloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    schema_version: u32,
    next_habit_id: u64,
    next_goal_id: u64,
    habits: Vec<Habit>,
    goals: Vec<Goal>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            next_habit_id: 1,
            next_goal_id: 1,
            habits: Vec::new(),
            goals: Vec::new(),
        }
    }
}
impl State {
    pub fn from_json(text: &str) -> Result<Self> {
        let state: Self =
            serde_json::from_str(text).map_err(|e| format!("invalid snapshot: {e}"))?;
        state.validate()?;
        Ok(state)
    }
    pub fn to_json(&self) -> Result<String> {
        self.validate()?;
        serde_json::to_string_pretty(self)
            .map(|s| s + "\n")
            .map_err(|e| e.to_string())
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(format!(
                "unsupported schema version {}",
                self.schema_version
            ));
        }
        let mut ids = BTreeSet::new();
        for h in &self.habits {
            h.validate()?;
            if !ids.insert(h.id()) || h.id() >= self.next_habit_id {
                return Err("invalid or duplicate habit id".into());
            }
        }
        if self.next_habit_id == 0 || self.next_goal_id == 0 {
            return Err("next ids must be positive".into());
        }
        ids.clear();
        for g in &self.goals {
            g.validate()?;
            if !ids.insert(g.id()) || g.id() >= self.next_goal_id {
                return Err("invalid or duplicate goal id".into());
            }
        }
        Ok(())
    }
    pub fn habits(&self) -> &[Habit] {
        &self.habits
    }
    pub fn goals(&self) -> &[Goal] {
        &self.goals
    }
    pub fn habit(&self, id: u64) -> Result<&Habit> {
        self.habits
            .iter()
            .find(|h| h.id() == id)
            .ok_or_else(|| format!("habit {id} not found"))
    }
    pub fn habit_mut(&mut self, id: u64) -> Result<&mut Habit> {
        self.habits
            .iter_mut()
            .find(|h| h.id() == id)
            .ok_or_else(|| format!("habit {id} not found"))
    }
    pub fn goal(&self, id: u64) -> Result<&Goal> {
        self.goals
            .iter()
            .find(|g| g.id() == id)
            .ok_or_else(|| format!("goal {id} not found"))
    }
    pub fn goal_mut(&mut self, id: u64) -> Result<&mut Goal> {
        self.goals
            .iter_mut()
            .find(|g| g.id() == id)
            .ok_or_else(|| format!("goal {id} not found"))
    }
    pub fn add_habit(&mut self, name: &str, created: Date) -> Result<u64> {
        let id = self.next_habit_id;
        let next = id.checked_add(1).ok_or("habit ids exhausted")?;
        let habit = Habit::new(id, name, created)?;
        self.habits.push(habit);
        self.next_habit_id = next;
        Ok(id)
    }
    pub fn add_goal(&mut self, name: &str, target: u64, unit: &str, created: Date) -> Result<u64> {
        let id = self.next_goal_id;
        let next = id.checked_add(1).ok_or("goal ids exhausted")?;
        let goal = Goal::new(id, name, target, unit, created)?;
        self.goals.push(goal);
        self.next_goal_id = next;
        Ok(id)
    }
}
