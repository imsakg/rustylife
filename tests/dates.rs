use rustylife::date::Date;

#[test]
fn parses_real_dates_and_rejects_noncanonical_inputs() {
    let leap: Date = "2020-02-29".parse().unwrap();
    assert_eq!(leap.to_string(), "2020-02-29");
    for invalid in [
        "2021-02-29",
        "2021-2-01",
        "2021-13-01",
        "0000-01-01",
        "2021-01-01x",
    ] {
        assert!(invalid.parse::<Date>().is_err(), "{invalid}");
    }
}

#[test]
fn shifts_dates_and_counts_calendar_days() {
    let date: Date = "2021-12-31".parse().unwrap();
    assert_eq!(date.add_days(1).unwrap().to_string(), "2022-01-01");
    assert_eq!(date.days_until("2022-01-03".parse().unwrap()), 3);
    assert!("9999-12-31".parse::<Date>().unwrap().add_days(1).is_err());
    assert!("0001-01-01".parse::<Date>().unwrap().add_days(-1).is_err());
}

#[test]
fn deserialization_uses_the_same_validation() {
    assert!(serde_json::from_str::<Date>("\"2021-02-29\"").is_err());
    let date: Date = serde_json::from_str("\"2022-03-11\"").unwrap();
    assert_eq!(serde_json::to_string(&date).unwrap(), "\"2022-03-11\"");
}
