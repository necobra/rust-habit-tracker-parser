use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug)]
pub struct HabitRecord {
    pub id: u32,
    pub description: String,
    pub goal_metric: f32,
    pub time_period: String,
    pub completion_status: Vec<bool>,
}

#[derive(Debug, Error)]
pub enum HabitParseError {
    #[error("Failed to parse habit record: {0}")]
    ParseError(String),
}

impl HabitRecord {
    pub fn from_record(record_str: &str) -> Result<Self, HabitParseError> {
        let pairs = Grammar::parse(Rule::habit_record, record_str)
            .map_err(|e| HabitParseError::ParseError(e.to_string()))?;

        let mut id = 0;
        let mut description = String::new();
        let mut goal_metric = 0.0;
        let mut time_period = String::new();
        let mut completion_status = Vec::new();

        for pair in pairs {
            if pair.as_rule() == Rule::habit_record {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::habit_number => {
                            id = inner_pair.as_str().parse().map_err(|_| {
                                HabitParseError::ParseError("Invalid habit number".to_string())
                            })?;
                        }
                        Rule::description => {
                            description = inner_pair.as_str().to_string();
                        }
                        Rule::goal_metric => {
                            goal_metric = inner_pair.as_str().parse().map_err(|_| {
                                HabitParseError::ParseError("Invalid goal metric".to_string())
                            })?;
                        }
                        Rule::time_period => {
                            time_period = inner_pair.as_str().parse().map_err(|_| {
                                HabitParseError::ParseError("Invalid time period".to_string())
                            })?;
                        }
                        Rule::completion_status => {
                            println!("{}", inner_pair);
                            for checkbox in inner_pair.into_inner() {
                                println!("{}", checkbox);
                                if let Some(inner_checkbox) = checkbox.into_inner().next() {
                                    match inner_checkbox.as_rule() {
                                        Rule::full_checkbox => completion_status.push(true),
                                        Rule::empty_checkbox => completion_status.push(false),
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(HabitRecord {
            id,
            description,
            goal_metric,
            time_period,
            completion_status,
        })
    }
}
