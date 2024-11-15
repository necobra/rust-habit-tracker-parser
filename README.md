# Habit Tracker Parser

- links: https://crates.io/crates/habit_tracker_parser
- docs: https://docs.rs/habit_tracker_parser/latest/habit_tracker_parser/

## Overview

The Habit Tracker Parser is a Rust-based application designed to parse and analyze data from the habit-tracking tool, Habito, which is widely used on Linux. This parser’s purpose is to convert raw habit-tracking records into a structured format, ready for further analysis or export. It simplifies the extraction of key metrics about habit performance, making it easy to observe patterns, visualize progress, and identify areas for improvement over time.

Key Features:
- Error handling - by using thiserror
- Testing
- CLI helper

## Grammar Rules

The parser will interpret each row of the raw Habito records using custom grammar rules that capture the following components:
1. **Habit Number and Description**: Each line begins with a habit's number and its description, separated by a colon and space.
   - Example: `9: write 15 pages` or `10: workout`
2. **Goal Metric**: Following the habit description is a metric representing the goal or frequency of the habit, recorded in decimal format.
   - Example: `15.0` for "write 15 pages" or `1.0` for "workout."
3. **Time Period**: This is a summary of the time period over which the habit has been tracked, usually represented as a string like "1 day" or "2 days."
4. **Completion Statuses**: The final part of each record is a list of boxes (□ and ■) representing each day's habit completion. An empty box (□) represents an incomplete day, while a filled box (■) signifies a completed day.
   - Example: `□ □ ■ □ □ □ □ □ □` indicates the habit was completed only once.

### Parsing Rules

The parser will recognize each part of the record based on positional patterns and character types:
- **Number & Description**: Parse up to the first colon `:` as an identifier and then capture the text until the metric.
- **Metric**: Identify as a float or decimal number immediately after the description.
- **Time Period**: A string following the metric with a unit indicating "day(s)."
- **Completion Statuses**: A sequence of empty `□` and filled `■` checkboxes, counted for calculating success rates.

## Parsing Example

Given an example record:

```
│ 9: write 15 pages │ 15.0 │ 1 days │ □ □ ■ □ □ □ □ □ □ │ 
│ 10: workout │ 1.0 │ 2 days │ ■ ■ □ □ □ □ □ □ □ │
```

The parser converts this into:

```
HabitRecord {
    id: 9,
    description: "write 15 pages",
    goal_metric: 15.0,
    time_period: "1 day",
    completion_status: vec![false, false, true, false, false, false, false, false, false],
}
```

```
HabitRecord {
    id: 10,
    description: "workout",
    goal_metric: 1.0,
    time_period: "2 days",
    completion_status: vec![true, true, false, false, false, false, false, false, false],
}
```