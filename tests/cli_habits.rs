use std::{path::Path, process::Command};
fn cli(path: &Path, args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_rustylife"))
        .arg("--data")
        .arg(path)
        .args(["--on", "2021-09-05"])
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}
#[test]
fn habit_commands_persist_across_processes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data.json");
    assert!(cli(&path, &["habit", "add", "Read"]).contains("habit 1"));
    assert!(cli(&path, &["habit", "check", "1"]).contains("Recorded"));
    assert!(cli(&path, &["habit", "check", "1"]).contains("Already"));
    assert!(cli(&path, &["habit", "list"]).contains("Read"));
    cli(&path, &["habit", "rename", "1", "Read books"]);
    assert!(cli(&path, &["habit", "list"]).contains("Read books"));
    cli(&path, &["habit", "archive", "1"]);
    assert!(!cli(&path, &["habit", "list"]).contains("Read books"));
    assert!(cli(&path, &["habit", "list", "all"]).contains("archived"));
    cli(&path, &["habit", "resume", "1"]);
    assert!(cli(&path, &["habit", "undo", "1"]).contains("Removed"));
}
