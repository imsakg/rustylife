# Snapshot format and safety contract

## Schema version 1

The data file is UTF-8 JSON with a trailing newline and these root fields:

- `schema_version`: currently `1`; unknown versions are rejected, not downgraded.
- `next_habit_id`, `next_goal_id`: independent positive integer allocators. Every
  existing ID must be smaller than its allocator, and IDs must be unique per kind.
- `habits`: `id`, `name`, `created`, `archived`, `category`, `checkins`.
- `goals`: `id`, `name`, `target`, `progress`, `unit`, `created`, `due`, `milestones`.

`category` is an optional string (older version-1 snapshots may omit it).
`due` is a nullable canonical date. Each milestone has `name` and `done`.
Check-ins serialize as a sorted set of canonical date strings. Empty habit and goal
arrays are valid. Unknown fields and invalid dates are errors. Names are trimmed at
creation; stored names must already be canonical. Targets and progress must satisfy
the same invariants as CLI inputs. JSON numbers must be exact integers, not floats.

Prefer application backups and explicit restore over hand-editing. Public domain
fields are private so normal library mutations preserve these invariants. The
`State::from_json`/`to_json` boundary validates complete snapshots. No automatic
schema migrations or lossy “best effort” imports are performed.

## Update protocol

1. Resolve the parent directory and acquire a nonblocking exclusive advisory lock
   on a separate `<data-file>.lock` file. It remains held for the Store lifetime.
2. Read at most 16 MiB plus one sentinel byte. Missing live data starts empty;
   missing restore input, malformed UTF-8/JSON, unsupported versions and invalid
   state are errors, never reasons to overwrite with empty state.
3. Apply a change to an in-memory copy. A returned error or panic publishes nothing.
   Mutations require exclusive `&mut Store` access, preventing races through a
   shared Store handle; cross-process handles are excluded by the file lock.
4. Validate and serialize, write a private temporary file in the same directory,
   flush it with `sync_all`, atomically replace the destination, then sync the
   directory on Unix. Temporary files clean themselves up on pre-replacement errors.
5. Explicitly unlock and close when the Store is dropped.

Backup uses atomic no-clobber persistence; restore first validates a bounded source
and then uses the normal replacement path. Restore intentionally does not parse the
old live file, allowing corrupt-file recovery. It does not keep an automatic undo
copy: create a backup first. A failure to sync the directory after replacement may
be reported even though the new bytes are already visible; inspect the file before
retrying an action with non-idempotent progress increments.

## Filesystem and privacy boundaries

- Unix snapshots and lock files are created mode `0600`. Data are **not encrypted**.
- Symlinks, directories, FIFOs and device paths are rejected for data/lock sources.
  Unix file opens also use nonblocking/no-follow flags and descriptor checks.
- Advisory locking only coordinates clients that honor it. Do not directly edit,
  replace or delete the lock file while another process holds it.
- Atomic replacement and durability depend on the filesystem. Use a local disk;
  network mounts and multi-host shared-folder editing are not supported.
- Windows behavior is covered by the portable API design, not by local macOS
  execution. Graphical simulation has independent native dependencies.
- This is not protection against a malicious process running as the same user or
  replacing parent directories; use OS permissions and backups for that boundary.
- The application performs no network synchronization. Cargo may fetch dependencies
  at build time. CSV exports expose names and dates; treat them like personal data.

## Tests

Integration tests cover rollback, corruption protection, size limits, symlink/FIFO
refusal, restrictive Unix permissions, lock contention/release, backup no-clobber,
restore validation, strict dates and end-to-end CLI workflows. A compile-fail
documentation test enforces exclusive access to update operations.
