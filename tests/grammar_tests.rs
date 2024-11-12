use anyhow::Result;
use habit_tracker_parser::{Grammar, HabitRecord, Rule};
use pest::Parser;

#[test]
fn test_parse_filled_checkbox() -> Result<()> {
    let pairs = Grammar::parse(Rule::checkbox, "■")?;
    assert_eq!(pairs.as_str(), "■");
    Ok(())
}

#[test]
fn test_parse_empty_checkbox() -> Result<()> {
    let pairs = Grammar::parse(Rule::checkbox, "□")?;
    assert_eq!(pairs.as_str(), "□");
    Ok(())
}

#[test]
fn test_parse_habit_number() -> Result<()> {
    let pairs = Grammar::parse(Rule::habit_number, "9")?;
    assert_eq!(pairs.as_str(), "9");
    Ok(())
}

#[test]
fn test_parse_description() -> Result<()> {
    let pairs = Grammar::parse(Rule::description, "write 15 pages")?;
    assert_eq!(pairs.as_str(), "write 15 pages");
    Ok(())
}

#[test]
fn test_parse_goal_metric() -> Result<()> {
    let pairs = Grammar::parse(Rule::goal_metric, "15.0")?;
    assert_eq!(pairs.as_str(), "15.0");
    Ok(())
}

#[test]
fn test_parse_time_period() -> Result<()> {
    let pairs = Grammar::parse(Rule::time_period, "1 days")?;
    assert_eq!(pairs.as_str(), "1 days");
    Ok(())
}

#[test]
fn test_parse_completion_status() -> Result<()> {
    let pairs = Grammar::parse(Rule::completion_status, "□ ■ □ □")?;
    assert_eq!(pairs.as_str(), "□ ■ □ □");
    Ok(())
}

#[test]
fn test_parse_full_habit_record() -> Result<()> {
    let record_str = "│ 9: write 15 pages │ 15.0 │ 1 days │ □ □ ■ □ □ □ □ □ □ │";
    let habit_record = HabitRecord::from_record(record_str).expect("Failed to parse habit record");

    assert_eq!(habit_record.id, 9);
    assert_eq!(habit_record.description, "write 15 pages ");
    assert_eq!(habit_record.goal_metric, 15.0);
    assert_eq!(habit_record.time_period, "1 days");
    assert_eq!(
        habit_record.completion_status,
        vec![false, false, true, false, false, false, false, false, false]
    );

    Ok(())
}
