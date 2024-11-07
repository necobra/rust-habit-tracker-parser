use habit_tracker_parser::{Grammar, Rule};
use pest::Parser;

fn main() -> anyhow::Result< () > {
    let input = "■";
    let pairs = Grammar::parse(Rule::checkbox, input)?;
    println!("{:?}", pairs);

    Ok(())
}
