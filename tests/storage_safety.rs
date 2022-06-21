use rustylife::store::Store;
#[test]
fn second_writer_is_rejected_until_first_handle_is_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("data.json");
    let store = Store::open(&p).unwrap();
    assert!(Store::open(&p).is_err());
    drop(store);
    assert!(Store::open(&p).is_ok());
}
#[test]
fn a_panicking_edit_never_publishes_its_partial_state() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("data.json");
    let mut store = Store::open(&p).unwrap();
    store.update(|_| Ok(())).unwrap();
    let before = std::fs::read(&p).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: rustylife::Result<()> = store.update(|s| {
            s.add_habit("Read", "2021-09-01".parse().unwrap())?;
            panic!("cancel")
        });
    }));
    assert!(result.is_err());
    assert_eq!(std::fs::read(&p).unwrap(), before);
    store
        .update(|s| s.add_habit("Walk", "2021-09-01".parse().unwrap()))
        .unwrap();
    assert_eq!(store.read().unwrap().habits().len(), 1);
}
#[test]
fn oversized_snapshots_are_rejected_without_overwriting() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("data.json");
    std::fs::File::create(&p)
        .unwrap()
        .set_len(rustylife::store::MAX_SNAPSHOT_BYTES + 1)
        .unwrap();
    let mut store = Store::open(&p).unwrap();
    assert!(store.read().is_err());
    assert!(store.update(|_| Ok(())).is_err());
    assert_eq!(
        std::fs::metadata(&p).unwrap().len(),
        rustylife::store::MAX_SNAPSHOT_BYTES + 1
    );
}
#[cfg(unix)]
#[test]
fn nonregular_data_and_lock_paths_are_rejected_before_opening() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("fifo.json");
    assert!(std::process::Command::new("mkfifo")
        .arg(&p)
        .status()
        .unwrap()
        .success());
    assert!(Store::open(&p).is_err());
    let p = dir.path().join("data.json");
    let lock = dir.path().join("data.json.lock");
    assert!(std::process::Command::new("mkfifo")
        .arg(&lock)
        .status()
        .unwrap()
        .success());
    assert!(Store::open(&p).is_err());
}
#[cfg(unix)]
#[test]
fn symlinks_are_rejected_and_new_snapshots_are_private() {
    use std::os::unix::{fs::symlink, fs::PermissionsExt};
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("data.json");
    let target = dir.path().join("target.json");
    std::fs::write(&target, "do not touch").unwrap();
    symlink(&target, &p).unwrap();
    assert!(Store::open(&p).is_err());
    std::fs::remove_file(&p).unwrap();
    let mut store = Store::open(&p).unwrap();
    store.update(|_| Ok(())).unwrap();
    assert_eq!(
        std::fs::metadata(&p).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert_eq!(std::fs::read_to_string(target).unwrap(), "do not touch");
}
