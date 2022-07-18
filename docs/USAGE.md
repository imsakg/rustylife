# Using rustylife

Commands below assume `rustylife` is installed. During development, replace it with
`cargo run --locked --`. Run `rustylife --help` for the exact command grammar.
Quote names containing spaces. Options must precede the command.

## Local data and dates

Path selection is `--data PATH`, then `RUSTYLIFE_DATA`, then
`$XDG_DATA_HOME/rustylife/data.json`, or `$HOME/.local/share/rustylife/data.json`.
`USERPROFILE` is accepted when `HOME` is absent. Empty explicit paths are errors.

`--on YYYY-MM-DD` selects the reference day for habit creation/check-ins, goal
creation, dashboards and reports. It defaults to today's local calendar date.
Dates are strictly canonical, Gregorian, and limited to years 0001–9999. Days
are calendar dates, not elapsed 24-hour intervals, so DST cannot split a check-in.

```sh
rustylife --data ./personal.json --on 2022-04-15 habit add "Evening walk"
rustylife --data ./personal.json --on 2022-04-15 habit check 1
```

The reference option does not travel back in time: names, archive state, categories
and goal totals are their current values. Explicit future habit dates are allowed
for planning/import-like entry, but streaks/reports ignore check-ins beyond their
reference day. Habit list totals and CSV show all stored dates.

## Habits

```sh
rustylife habit add "Read for ten minutes"
rustylife habit check 1
rustylife habit undo 1
rustylife habit rename 1 "Read before bed"
rustylife habit category 1 Learning
rustylife habit list category Learning
rustylife habit category 1 none
rustylife habit archive 1
rustylife habit list all
rustylife habit resume 1
rustylife habit calendar 1 2022-04-01 2022-04-30
```

- A habit is expected every day from its creation date. Check-ins before creation
  are rejected; repeated check-ins return “Already recorded” and do not add counts.
- Undo only removes the selected day and is safe to repeat, even when archived.
- Archive hides a habit from dashboards, active lists and reviews. It retains its
  dates; resume restores the habit. There is no scheduled pause or archive timeline.
- Categories are optional, normalized at their edges, and matched case-sensitively.
  CLI `none` removes a category; category details are retained in JSON backups.
- Names, units, milestones and categories must contain 1–120 printable characters.
- Current streak ends today, or yesterday if today has not yet been checked in.
  A longer gap resets it. Best streak only considers days through the reference.
- Calendar rows are seven-day chunks starting at the supplied start, **not** Monday-
  aligned weeks. `#` means done, `.` missed and space before creation; 366 days max.

## Goals and milestones

```sh
rustylife goal add "Read twelve books" 12 books 2022-12-31
rustylife goal advance 1 3
rustylife goal set 1 2
rustylife goal due 1 2023-01-31
rustylife goal due 1 none
rustylife goal milestone 1 add "Choose the next book"
rustylife goal milestone 1 done 1
rustylife goal milestone 1 undo 1
rustylife goal list
```

Targets are positive whole `u64` units; progress is a nonnegative total no greater
than the target. Out-of-range values and addition overflow are errors, not clamps.
A goal is complete when its progress equals its target; otherwise it becomes
overdue **after** the due day. Deadlines cannot precede creation. Correcting progress
below target reopens completion. Milestones have one-based positions, unique names,
and a 100-step limit; checking a milestone never silently changes numeric progress.

## Reviews and exports

```sh
rustylife today
rustylife --on 2022-04-17 report weekly
rustylife --on 2022-04-30 report monthly
rustylife export habits > habit-checkins.csv
rustylife export goals > goals.csv
```

Weekly means seven inclusive days ending at `--on`; monthly means the first of its
calendar month through `--on`. Select the month's final day for a complete month.
Habit eligibility is clipped to creation; totals are weighted by eligible days,
not an average of percentages. Archived habits are excluded by current state,
so a past report is not a historical membership snapshot. No eligible days yields
0%, not a division error. Goal rows explicitly show current totals, not historic
progress. ASCII bars use `#` and `-`, floor percentages, and never exceed 80 cells.

Habit CSV has one row per stored check-in, including archived/future entries.
Habits with no check-ins produce no rows. Goal CSV has one row per goal and includes
milestone counts. Fields follow CSV quoting rules and CRLF row endings. Formula-
like text is prefixed with an apostrophe for safer spreadsheet opening; use JSON
backups when exact unmodified strings matter. **Never redirect export output onto
the active JSON path**: the shell truncates destinations before the app starts.

## Backup, restore, and failures

```sh
rustylife backup ./life-2022-04-30.json
rustylife restore ./life-2022-04-30.json --replace
```

A backup destination's parent must already exist. Existing files are never
clobbered. Restore validates the complete source first and replaces the whole live
snapshot, not a merge. `--replace` is mandatory; back up first if you need the
current state. Restore works even when the active snapshot is corrupt. Source
and destination cannot be the active path itself. Neither operation uploads data.

Successful commands exit 0. Argument, validation, store-lock and data errors exit 2
with an explanation on stderr and no success output; stdout I/O errors exit 1.
Help/version do not create data directories. Read commands may create the parent
and lock file but do not publish an empty snapshot. On “busy”, close the other
writer and retry. Never delete a live lock file to bypass it; file presence alone
does not mean the lock is held.
