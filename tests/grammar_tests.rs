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
fn test_parse_goal_metric_integer() -> Result<()> {
    let pairs = Grammar::parse(Rule::goal_metric, "15")?;
    assert_eq!(pairs.as_str(), "15");
    Ok(())
}

#[test]
fn test_parse_goal_metric_decimal() -> Result<()> {
    let pairs = Grammar::parse(Rule::goal_metric, "15.0")?;
    assert_eq!(pairs.as_str(), "15.0");
    Ok(())
}

#[test]
fn test_parse_time_period_days() -> Result<()> {
    let pairs = Grammar::parse(Rule::time_period, "1 days")?;
    assert_eq!(pairs.as_str(), "1 days");
    Ok(())
}

#[test]
fn test_parse_time_period_weeks() -> Result<()> {
    let pairs = Grammar::parse(Rule::time_period, "2 weeks")?;
    assert_eq!(pairs.as_str(), "2 weeks");
    Ok(())
}

#[test]
fn test_parse_category() -> Result<()> {
    let pairs = Grammar::parse(Rule::optional_category, "[fitness]")?;
    assert_eq!(pairs.as_str(), "[fitness]");
    Ok(())
}

#[test]
fn test_parse_tags() -> Result<()> {
    let pairs = Grammar::parse(Rule::optional_tags, "{health, productivity}")?;
    assert_eq!(pairs.as_str(), "{health, productivity}");
    Ok(())
}

#[test]
fn test_parse_completion_status() -> Result<()> {
    let pairs = Grammar::parse(Rule::completion_status, "□ ■ □ □")?;
    assert_eq!(pairs.as_str(), "□ ■ □ □");
    Ok(())
}

#[test]
fn test_parse_full_habit_record_with_category_and_tags() -> Result<()> {
    let record_str = "│ 10 │ workout │ 1 │ 2 weeks │ [fitness] │ {health, strength} │ ■ ■ □ □ □ □ □ □ □ │";
    let habit_record = HabitRecord::from_record(record_str).expect("Failed to parse habit record");

    assert_eq!(habit_record.id, 10);
    assert_eq!(habit_record.description, "workout ");
    assert_eq!(habit_record.goal_metric, 1.0);
    assert_eq!(habit_record.time_period, "2 weeks");
    assert_eq!(habit_record.optional_category, Some("fitness".to_string()));
    assert_eq!(
        habit_record.optional_tags,
        Some(vec!["health".to_string(), "strength".to_string()])
    );
    assert_eq!(
        habit_record.completion_status,
        vec![true, true, false, false, false, false, false, false, false]
    );

    Ok(())
}
