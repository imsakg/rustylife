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
            unit: valid_name(unit)?,
            created,
        })
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
