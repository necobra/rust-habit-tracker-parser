use habit_tracker_parser::HabitRecord;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
        print_help();
        return Ok(());
    }

    let record = args[1].as_str();
    match HabitRecord::from_record(record) {
        Ok(parsed) => println!("{:#?}", parsed),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

fn print_help() {
    println!(
        r#"
Habit Tracker Parser

Usage:
    habit_tracker_parser "<habit_record_string>"
    
Arguments:
    --help                  Show this help message.

Example:
    habit_tracker_parser "9: write 15 pages | 15.0 | 1 day | □ □ ■ □ □ □ □ □ □"

Description:
    Parses a habit record string into a structured format, extracting:
    - Habit ID
    - Description
    - Goal Metric
    - Time Period
    - Completion Status (□ for incomplete, ■ for complete)

Output:
    Prints the parsed record as a structured format or an error message if parsing fails.
"#
    );
}
