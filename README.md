# rustylife

Local-first habit tracking and measurable goals, with deterministic terminal
reviews. No accounts, server, telemetry, or network calls in the life-management
application.

## Start

```sh
cargo run --locked -- --help
cargo run --locked -- habit add "Read for ten minutes"
cargo run --locked -- habit check 1
cargo run --locked -- goal add "Read twelve books" 12 books
cargo run --locked -- goal advance 1 3
cargo run --locked -- today
cargo run --locked -- report weekly
```

IDs in the example assume a new data file. Use `--data PATH` before the command to
keep separate journals; the default is local user storage, not this checkout.
To install only the CLI: `cargo install --locked --path . --bin rustylife`.

## Included

- Daily habits: unique check-ins, undo, rename, archive/resume, category filters.
- Goals: whole-unit targets, progress correction, deadlines, reversible milestones.
- Progress: streaks, bounded ASCII bars, seven-day-row check-in calendars.
- Reviews: daily dashboard, rolling weekly and calendar month-to-date reports.
- Persistence: versioned validated JSON, advisory writer lock, atomic file replacement.
- Portability: non-overwriting backups, explicit validated restore, escaped CSV exports.

[Usage and semantics](docs/USAGE.md) · [Data and safety](docs/DATA.md) ·
[Implementation plan](docs/PLAN.md) · [History provenance](docs/HISTORY.md)

## Verify

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo test --locked --doc
```

The optional original graphics experiment is still in `src/main.rs`:
`cargo run --features simulation --bin smoothlife` (run from this directory so
`smoothlife.fs` is available). It requires raylib's native toolchain and a display;
headless CLI checks do not validate its graphical runtime.

## Boundaries

Daily schedules only; goal progress is an integer total, not an event history.
Reports use the current active habit set and current goal totals, even when a past
reference date is selected. No reminders, synchronization, encryption, or accounts.
Back up valuable data separately. Reconstructed Git timestamps are explicitly
explained in [history provenance](docs/HISTORY.md), not represented as real activity.
