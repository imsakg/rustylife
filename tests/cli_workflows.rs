use std::{
    path::Path,
    process::{Command, Output},
};

fn invoke(path: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rustylife"))
        .arg("--data")
        .arg(path)
        .args(["--on", "2021-09-07"])
        .args(args)
        .output()
        .unwrap()
}
fn ok(path: &Path, args: &[&str]) -> String {
    let out = invoke(path, args);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stderr.is_empty());
    String::from_utf8(out.stdout).unwrap()
}
#[test]
fn complete_offline_workflow_covers_reports_exports_backup_and_restore() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("life.json");
    let b = dir.path().join("backup.json");
    ok(&p, &["habit", "add", "Read"]);
    ok(&p, &["habit", "check", "1"]);
    ok(&p, &["goal", "add", "Books", "12", "books", "2021-12-31"]);
    ok(&p, &["goal", "advance", "1", "3"]);
    assert!(ok(&p, &["today"]).contains("1/1 habits completed"));
    assert!(ok(&p, &["report", "weekly"]).contains("Total: 1/1 (100.0%)"));
    assert!(ok(&p, &["report", "monthly"]).contains("Read: 1/1"));
    assert!(ok(&p, &["habit", "calendar", "1", "2021-09-01", "2021-09-07"]).contains("      #"));
    assert!(ok(&p, &["export", "habits"]).contains("1,Read,2021-09-07,false"));
    assert!(ok(&p, &["export", "goals"]).contains("1,Books,3,12,books,2021-12-31,active"));
    ok(&p, &["backup", b.to_str().unwrap()]);
    ok(&p, &["habit", "undo", "1"]);
    ok(&p, &["restore", b.to_str().unwrap(), "--replace"]);
    assert!(ok(&p, &["today"]).contains("1/1 habits completed"));
}
#[test]
fn invalid_commands_have_exit_two_and_never_modify_existing_data() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("life.json");
    ok(&p, &["habit", "add", "Read"]);
    let before = std::fs::read(&p).unwrap();
    for args in [
        vec!["not-a-command"],
        vec!["habit", "check", "0"],
        vec!["habit", "check", "99"],
        vec!["habit", "add", " "],
        vec!["goal", "add", "Books", "0", "books"],
        vec!["goal", "set", "1", "-1"],
        vec!["restore", "not-a-backup"],
        vec!["habit", "list", "extra"],
    ] {
        let out = invoke(&p, &args);
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        assert!(out.stdout.is_empty());
        assert!(!out.stderr.is_empty());
        assert_eq!(std::fs::read(&p).unwrap(), before);
    }
}
#[test]
fn help_version_and_readonly_reports_do_not_publish_snapshots() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("new/data.json");
    ok(&p, &["--help"]);
    ok(&p, &["--version"]);
    assert!(!p.parent().unwrap().exists());
    ok(&p, &["today"]);
    ok(&p, &["report", "weekly"]);
    ok(&p, &["export", "goals"]);
    assert!(!p.exists());
    ok(&p, &["habit", "add", "Read"]);
    let before = std::fs::read(&p).unwrap();
    ok(&p, &["today"]);
    ok(&p, &["report", "monthly"]);
    assert_eq!(std::fs::read(&p).unwrap(), before);
}
#[test]
fn invalid_reference_dates_are_reported_instead_of_panicking() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("life.json");
    for date in ["2021-02-29", "tomorrow", "2021-9-7", "0000-01-01"] {
        let out = Command::new(env!("CARGO_BIN_EXE_rustylife"))
            .arg("--data")
            .arg(&p)
            .args(["--on", date, "today"])
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(2));
        assert!(!p.exists());
    }
}
#[test]
fn explicit_data_path_beats_environment_and_environment_beats_default() {
    let dir = tempfile::tempdir().unwrap();
    let explicit = dir.path().join("explicit.json");
    let env = dir.path().join("env.json");
    let out = Command::new(env!("CARGO_BIN_EXE_rustylife"))
        .env("RUSTYLIFE_DATA", &env)
        .arg("--data")
        .arg(&explicit)
        .args(["habit", "add", "Read"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(explicit.exists());
    assert!(!env.exists());
    let out = Command::new(env!("CARGO_BIN_EXE_rustylife"))
        .env("RUSTYLIFE_DATA", &env)
        .args(["habit", "add", "Walk"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(env.exists());
}
#[test]
fn corrupt_data_and_live_lock_have_actionable_failures() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("life.json");
    std::fs::write(&p, "not json").unwrap();
    let out = invoke(&p, &["today"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("snapshot"));
    assert_eq!(std::fs::read_to_string(&p).unwrap(), "not json");
    let _held = rustylife::store::Store::open(&p).unwrap();
    let out = invoke(&p, &["today"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("busy"));
}
