use rustylife::store::Store;

#[test]
fn transactions_persist_and_failed_edits_roll_back() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nested/data.json");
    {
        let store = Store::open(&path).unwrap();
        assert!(store.read().unwrap().habits().is_empty());
        store
            .update(|s| s.add_habit("Read", "2021-09-01".parse().unwrap()))
            .unwrap();
        let before = std::fs::read(&path).unwrap();
        let result = store.update(|s| {
            s.add_habit("Walk", "2021-09-01".parse().unwrap())?;
            Err::<(), _>("cancelled".into())
        });
        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), before);
    }
    let store = Store::open(&path).unwrap();
    assert_eq!(store.read().unwrap().habits().len(), 1);
    assert_eq!(store.read().unwrap().habit(1).unwrap().name(), "Read");
}

#[test]
fn corruption_is_an_error_not_a_blank_state() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data.json");
    std::fs::write(&path, b"broken snapshot").unwrap();
    let store = Store::open(&path).unwrap();
    assert!(store.read().is_err());
    assert!(store.update(|_| Ok(())).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), b"broken snapshot");
}
