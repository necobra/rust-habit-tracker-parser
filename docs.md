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