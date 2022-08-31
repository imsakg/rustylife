use crate::{date::Date, goal::Goal, habit::Habit, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub const SCHEMA_VERSION: u32 = 1;
pub const MAX_SNAPSHOT_BYTES: u64 = 16 * 1024 * 1024;

/// Holds a nonblocking exclusive advisory lock for a complete read/edit/write cycle.
/// Dropping this handle releases the lock, including on error or panic.
pub struct Store {
    path: PathBuf,
    _lock: File,
}
impl Drop for Store {
    fn drop(&mut self) {
        let _ = fs2::FileExt::unlock(&self._lock);
    }
}
impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let name = path.file_name().ok_or("data path must name a file")?;
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        fs::create_dir_all(parent).map_err(|e| format!("create data directory: {e}"))?;
        let parent = parent.canonicalize().map_err(|e| e.to_string())?;
        let path = parent.join(name);
        reject_symlink(&path)?;
        let mut lock_name = name.to_os_string();
        lock_name.push(".lock");
        let lock_path = parent.join(lock_name);
        reject_symlink(&lock_path)?;
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options
                .mode(0o600)
                .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
        }
        let lock = options
            .open(lock_path)
            .map_err(|e| format!("open data lock: {e}"))?;
        if !lock.metadata().map_err(|e| e.to_string())?.is_file() {
            return Err("lock path is not a regular file".into());
        }
        fs2::FileExt::try_lock_exclusive(&lock)
            .map_err(|e| format!("data store is busy or cannot be locked: {e}"))?;
        Ok(Self { path, _lock: lock })
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn read(&self) -> Result<State> {
        read_snapshot(&self.path, true)
    }
    /// Creates a new backup; existing destinations are never overwritten.
    pub fn backup(&self, destination: impl AsRef<Path>) -> Result<()> {
        let path = absolute_target(destination.as_ref())?;
        if path == self.path {
            return Err("backup destination is the active data file".into());
        }
        write_snapshot(&path, &self.read()?, false)
    }
    /// Replace the active snapshot with an independently validated backup.
    pub fn restore(&mut self, source: impl AsRef<Path>) -> Result<()> {
        let path = absolute_target(source.as_ref())?;
        if path == self.path {
            return Err("restore source is the active data file".into());
        }
        let state = read_snapshot(&path, false)?;
        write_snapshot(&self.path, &state, true)
    }
    /// Requires exclusive access, so two threads cannot race via one Store handle.
    ///
    /// ```compile_fail
    /// use rustylife::store::Store;
    /// use std::sync::Arc;
    /// let shared = Arc::new(Store::open("data.json").unwrap());
    /// shared.update(|_| Ok(())); // Arc does not provide exclusive mutable access.
    /// ```
    pub fn update<T>(&mut self, change: impl FnOnce(&mut State) -> Result<T>) -> Result<T> {
        let mut state = self.read()?;
        let output = change(&mut state)?;
        write_snapshot(&self.path, &state, true)?;
        Ok(output)
    }
}
fn absolute_target(path: &Path) -> Result<PathBuf> {
    let name = path.file_name().ok_or("snapshot path must name a file")?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    Ok(parent
        .canonicalize()
        .map_err(|e| format!("snapshot directory: {e}"))?
        .join(name))
}
fn read_snapshot(path: &Path, missing_is_empty: bool) -> Result<State> {
    reject_symlink(path)?;
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = match options.open(path) {
        Ok(file) => file,
        Err(e) if missing_is_empty && e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(State::default())
        }
        Err(e) => return Err(format!("open snapshot: {e}")),
    };
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err("snapshot is not a regular file".into());
    }
    let mut bytes = Vec::new();
    file.take(MAX_SNAPSHOT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("read snapshot: {e}"))?;
    if bytes.len() as u64 > MAX_SNAPSHOT_BYTES {
        return Err("snapshot exceeds the 16 MiB limit".into());
    }
    let text = std::str::from_utf8(&bytes).map_err(|e| format!("snapshot is not UTF-8: {e}"))?;
    State::from_json(text)
}

fn reject_symlink(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.file_type().is_symlink() => {
            Err(format!("refusing symbolic link: {}", path.display()))
        }
        Ok(meta) if !meta.is_file() => Err(format!("not a regular file: {}", path.display())),
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("inspect {}: {e}", path.display())),
    }
}
fn write_snapshot(path: &Path, state: &State, overwrite: bool) -> Result<()> {
    let text = state.to_json()?;
    if text.len() as u64 > MAX_SNAPSHOT_BYTES {
        return Err("snapshot exceeds the 16 MiB limit".into());
    }
    reject_symlink(path)?;
    let parent = path.parent().ok_or("snapshot has no parent directory")?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)
        .map_err(|e| format!("create temporary snapshot: {e}"))?;
    temp.write_all(text.as_bytes())
        .map_err(|e| format!("write snapshot: {e}"))?;
    temp.as_file()
        .sync_all()
        .map_err(|e| format!("sync snapshot: {e}"))?;
    if overwrite {
        temp.persist(path)
            .map_err(|e| format!("replace snapshot: {e}"))?;
    } else {
        temp.persist_noclobber(path)
            .map_err(|e| format!("create backup without overwriting: {e}"))?;
    }
    #[cfg(unix)]
    File::open(parent)
        .and_then(|dir| dir.sync_all())
        .map_err(|e| format!("sync data directory: {e}"))?;
    Ok(())
}

/// Portable, versioned application state. IDs remain stable across reloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawState")]
pub struct State {
    schema_version: u32,
    next_habit_id: u64,
    next_goal_id: u64,
    habits: Vec<Habit>,
    goals: Vec<Goal>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawState {
    schema_version: u32,
    next_habit_id: u64,
    next_goal_id: u64,
    habits: Vec<Habit>,
    goals: Vec<Goal>,
}
impl TryFrom<RawState> for State {
    type Error = String;
    fn try_from(raw: RawState) -> Result<Self> {
        let state = Self {
            schema_version: raw.schema_version,
            next_habit_id: raw.next_habit_id,
            next_goal_id: raw.next_goal_id,
            habits: raw.habits,
            goals: raw.goals,
        };
        state.validate()?;
        Ok(state)
    }
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
