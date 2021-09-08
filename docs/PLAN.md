# Life-management implementation plan

> **For Hermes:** Follow the test-driven-development workflow for each behavior;
> use an independent read-only review before the final verification.

**Goal:** Deliver a usable offline habit and goal manager in thirty coherent changes.

**Architecture:** A pure Rust domain library feeds a deterministic CLI. Dates and
identifiers are validated at boundaries. Versioned JSON is loaded under an advisory
lock and replaced atomically; report generation never modifies source data. Keep
the SmoothLife binary opt-in so CLI users need no graphics toolchain.

**Tech stack:** Rust 2021, chrono, serde/serde_json, fs2, standard-library CLI,
Cargo integration tests.

## Commit-sized sequence

For every behavior: add the named test, run it and observe RED, implement the
smallest coherent API, run all default-feature tests, then commit exact paths.
Documentation and build-only steps use their relevant parser/build verification.

1. Document scope, implementation sequence and reconstructed-history provenance.
2. Separate the optional graphics binary from a headless CLI (`Cargo.toml`, `src/bin/rustylife.rs`; help smoke test).
3. Add validated calendar dates (`src/date.rs`; leap-day and range tests).
4. Create validated daily habits (`src/habit.rs`; name and identifier tests).
5. Record idempotent dated check-ins (`tests/habits.rs`).
6. Undo mistaken check-ins without deleting history (`tests/habits.rs`).
7. Rename and archive habits (`tests/habits.rs`).
8. Calculate current and longest streaks (`tests/habits.rs`).
9. Calculate bounded completion rates (`tests/habits.rs`).
10. Define measurable goals (`src/goal.rs`; invalid-target tests).
11. Advance and correct goal progress (`tests/goals.rs`).
12. Track goal due dates and completion status (`tests/goals.rs`).
13. Add and complete goal milestones (`tests/goals.rs`).
14. Persist validated versioned JSON (`src/store.rs`; round-trip/corruption tests).
15. Add locked transactions and atomic replacement (`tests/store.rs`).
16. Wire habit commands to local persistence (`src/cli.rs`; process integration tests).
17. Wire goal commands and milestone editing (`tests/cli.rs`).
18. Render safe terminal progress bars (`src/visual.rs`; exact-output tests).
19. Visualize habit calendars (`tests/visual.rs`).
20. Assemble a deterministic daily dashboard (`src/report.rs`; snapshot assertions).
21. Generate weekly reviews (`tests/reports.rs`).
22. Generate monthly reviews (`tests/reports.rs`).
23. Export correctly escaped CSV (`src/export.rs`; escaping tests).
24. Back up and restore validated snapshots (`tests/store.rs`).
25. Categorize and filter habits (`tests/habits.rs`, `tests/cli.rs`).
26. Strengthen persistence regression coverage and corruption protection (`tests/store.rs`).
27. Exercise complete CLI workflows and error paths (`tests/cli.rs`).
28. Write usage and storage-format documentation (`docs/USAGE.md`, `docs/DATA.md`).
29. Add CI gates and a reproducible example workflow (`.github/workflows/ci.yml`, `examples/`).
30. Resolve final review findings, record release notes and verified limitations.

## Acceptance and deferrals

- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --all-targets`, `cargo test --doc`, and a release CLI smoke pass.
- Validate corrupted data is rejected without overwrite; duplicate check-ins do
  not inflate streaks, and failed mutations leave the previous snapshot intact.
- Count exactly 31 reachable commits. Validate both date fields are monotonic,
  weekday evenings within the requested interval after a local root rebase.
- Preserve the pre-existing uncommitted graphics edit and origin refs; never push.
- Defer accounts, synchronization, notifications, GUI life management, scheduled
  habits and period-correct toolchain support. Graphics requires native dependencies
  and is verified separately when those are available.
