use rustylife::{
    export::{csv_field, goal_csv, habit_csv},
    store::State,
};
#[test]
fn csv_escapes_delimiters_quotes_newlines_and_spreadsheet_formulas() {
    assert_eq!(csv_field("a,b"), "\"a,b\"");
    assert_eq!(csv_field("a\"b"), "\"a\"\"b\"");
    assert_eq!(csv_field("a\nb"), "\"a\nb\"");
    assert_eq!(csv_field("=1+1"), "'=1+1");
    assert_eq!(csv_field("  @danger"), "'  @danger");
}
#[test]
fn exports_preserve_archived_checkins_and_current_goal_totals() {
    let mut s = State::default();
    let date = "2021-09-01".parse().unwrap();
    s.add_habit("Read, reflect", date).unwrap();
    s.habit_mut(1).unwrap().check_in(date).unwrap();
    s.habit_mut(1).unwrap().set_archived(true);
    assert_eq!(
        habit_csv(&s),
        "habit_id,habit_name,date,archived\r\n1,\"Read, reflect\",2021-09-01,true\r\n"
    );
    s.add_goal("Books", 12, "books", date).unwrap();
    s.goal_mut(1).unwrap().advance(3).unwrap();
    assert!(goal_csv(&s, date).contains("1,Books,3,12,books,,active,0,0\r\n"));
}
