use std::process::Command;

#[test]
fn help_runs_without_a_graphics_context() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustylife"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("habits and goals"));
}
