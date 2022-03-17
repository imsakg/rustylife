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
fn goals_and_milestones_can_be_managed_from_separate_invocations() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("data.json");
    assert!(cli(&p, &["goal", "add", "Books", "12", "books", "2021-12-31"]).contains("goal 1"));
    cli(&p, &["goal", "advance", "1", "3"]);
    assert!(cli(&p, &["goal", "list"]).contains("3/12 books"));
    cli(&p, &["goal", "set", "1", "0"]);
    cli(&p, &["goal", "due", "1", "none"]);
    cli(&p, &["goal", "milestone", "1", "add", "Choose books"]);
    cli(&p, &["goal", "milestone", "1", "done", "1"]);
    assert!(cli(&p, &["goal", "list"]).contains("[x] 1: Choose books"));
    cli(&p, &["goal", "milestone", "1", "undo", "1"]);
    assert!(cli(&p, &["goal", "list"]).contains("[ ] 1: Choose books"));
}
