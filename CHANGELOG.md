# Changelog

## 0.1.0 — offline life-management baseline

This implementation was built and verified in October 2026. Local history uses
owner-requested reconstructed dates; see [history provenance](docs/HISTORY.md).

### Delivered

- Daily habit lifecycle, unique check-ins, corrections, categories and streaks.
- Measurable goals with safe progress arithmetic, deadlines and named milestones.
- Terminal dashboard, ASCII bars, bounded calendars and weekly/monthly reviews.
- Versioned JSON, strict input validation, locked atomic writes, backups/restore.
- Analysis CSV with correct quoting and spreadsheet formula neutralization.
- CLI integration suites, documentation and a disposable release-binary demo.
- Portable CI definition for Linux, macOS and Windows (not remotely run here).

### Review hardening

Independent review identified shared-handle concurrent updates, domain Serde
validation bypasses and FIFO reads as concrete risks. Updates/restore now require
exclusive mutable access; a compile-fail test enforces the shared-handle boundary.
Public domain deserialization uses validated wire representations. Non-regular
files are refused, with Unix no-follow/nonblocking opens and descriptor checks.
Explicit lock release also avoids inherited-descriptor timing on Unix process
spawns. Regression tests exercise these boundaries and data-preserving failures.

### Local verification

On macOS with Rust 1.99.0:

- `cargo fmt --check` and strict all-target Clippy pass for default features.
- 49 integration tests plus one compile-fail documentation test pass.
- The release-binary example exercises persistence, reports, CSV and byte-exact
  backup recovery in a disposable journal.
- `git diff --check` passes.

The original optional simulation source is preserved. Its separate check was
attempted and is blocked by the host's missing `raylib.h`; no native dependencies
were installed. Graphical runtime, remote CI and Windows/Linux execution were not
verified locally. Headless life-management builds need no raylib installation.

### Intentional limitations

Daily schedules only; no reminder service, accounts, synchronization or encryption.
Goal totals and active/archived membership are current-state snapshots, not an
event-sourced historical model. CSV is not a lossless backup. Filesystem guarantees
assume a cooperating local-disk client, not hostile same-user processes or network
mounts. See [data safety](docs/DATA.md) before manual edits or recovery.
