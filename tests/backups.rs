use rustylife::store::Store;
#[test]
fn backups_are_validated_nonclobbering_and_restore_can_recover_corruption() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("data.json");
    let backup = dir.path().join("backup.json");
    let mut store = Store::open(&p).unwrap();
    store
        .update(|s| s.add_habit("Read", "2021-09-01".parse().unwrap()))
        .unwrap();
    store.backup(&backup).unwrap();
    assert!(store.backup(&backup).is_err());
    assert!(store.backup(&p).is_err());
    store
        .update(|s| s.add_habit("Walk", "2021-09-01".parse().unwrap()))
        .unwrap();
    assert_eq!(store.read().unwrap().habits().len(), 2);
    store.restore(&backup).unwrap();
    assert_eq!(store.read().unwrap().habits().len(), 1);
    std::fs::write(&p, b"corrupted").unwrap();
    store.restore(&backup).unwrap();
    assert_eq!(store.read().unwrap().habits().len(), 1);
    assert!(store.restore(&p).is_err());
    let before = std::fs::read(&p).unwrap();
    assert!(store.restore(dir.path().join("missing.json")).is_err());
    std::fs::write(&backup, b"invalid").unwrap();
    assert!(store.restore(&backup).is_err());
    assert_eq!(std::fs::read(&p).unwrap(), before);
}
