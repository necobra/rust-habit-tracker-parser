use habit_tracker_parser::HabitRecord;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("please provide the text for parsing as an argument");
        return Ok(());
    }

    let record = args[1].as_str();
    match HabitRecord::from_record(record) {
        Ok(parsed) => println!("{:#?}", parsed),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
