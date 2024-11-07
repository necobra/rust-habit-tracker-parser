use anyhow::Result;
use habit_tracker_parser::{Grammar, Rule};
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
fn test_parse_invalid_input() {
    let result = Grammar::parse(Rule::checkbox, "X");
    assert!(result.is_err());
}
