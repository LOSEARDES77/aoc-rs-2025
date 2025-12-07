advent_of_code::solution!(6);

enum Operation {
    Add,
    Mult,
}

impl Operation {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operation::Add),
            '*' => Some(Operation::Mult),
            _ => None,
        }
    }
}

struct GridCol {
    numbers: Vec<u64>,
    operation: Operation,
}

impl GridCol {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Mult => self.numbers.iter().product(),
        }
    }
}

fn parse_input(input: &str) -> Vec<GridCol> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    let width = lines[0].len();
    let mut columns = vec![];

    // Find column positions by looking at non-space characters
    let mut col_positions = vec![];
    for col in 0..width {
        let mut has_content = false;
        for line in &lines {
            if let Some(ch) = line.chars().nth(col)
                && ch != ' '
            {
                has_content = true;
                break;
            }
        }
        if has_content {
            col_positions.push(col);
        }
    }

    // Group consecutive positions into column ranges
    let mut column_ranges = vec![];
    if !col_positions.is_empty() {
        let mut start = col_positions[0];
        let mut end = start;

        for &pos in &col_positions[1..] {
            if pos == end + 1 {
                end = pos;
            } else {
                column_ranges.push((start, end));
                start = pos;
                end = pos;
            }
        }
        column_ranges.push((start, end));
    }

    // Parse each column
    for (start_col, end_col) in column_ranges {
        let mut numbers = vec![];
        let mut operation = None;

        for line in &lines {
            let col_text: String = line
                .chars()
                .skip(start_col)
                .take(end_col - start_col + 1)
                .collect::<String>()
                .trim()
                .to_string();

            if col_text.is_empty() {
                continue;
            }

            // Try to parse as number first
            if let Ok(num) = col_text.parse::<u64>() {
                numbers.push(num);
            } else if col_text.len() == 1 {
                // Try to parse as operation
                if let Some(op) = Operation::from_char(col_text.chars().next().unwrap()) {
                    operation = Some(op);
                }
            }
        }

        if let Some(op) = operation {
            columns.push(GridCol {
                numbers,
                operation: op,
            });
        }
    }

    columns
}

pub fn part_one(input: &str) -> Option<u64> {
    let columns = parse_input(input);
    let total: u64 = columns.iter().map(|col| col.solve()).sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Some(0);
    }

    // Find the maximum width across all rows
    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let num_rows = lines.len() - 1; // Exclude operation row

    // Find column groups separated by spaces
    let mut col_groups = Vec::new();
    let mut current_group = Vec::new();

    for col in 0..max_width {
        let mut all_spaces = true;
        for line in &lines {
            if let Some(ch) = line.chars().nth(col)
                && ch != ' '
            {
                all_spaces = false;
                break;
            }
        }

        if all_spaces && !current_group.is_empty() {
            col_groups.push(current_group.clone());
            current_group.clear();
        } else if !all_spaces {
            current_group.push(col);
        }
    }

    if !current_group.is_empty() {
        col_groups.push(current_group);
    }

    let mut problems = vec![];

    // For each group, extract the operation and numbers
    for group in col_groups {
        // Get operation for this group
        let mut operation = None;
        for &col in &group {
            if let Some(ch) = lines.last().unwrap().chars().nth(col)
                && let Some(op) = Operation::from_char(ch)
            {
                operation = Some(op);
                break;
            }
        }

        if let Some(op) = operation {
            // For each column position in this group, read vertically to form numbers
            let mut numbers = vec![];
            for &col in &group {
                let mut number_str = String::new();
                for row in 0..num_rows {
                    if let Some(ch) = lines[row].chars().nth(col)
                        && ch.is_ascii_digit()
                    {
                        number_str.push(ch);
                    }
                }
                if !number_str.is_empty()
                    && let Ok(num) = number_str.parse::<u64>()
                {
                    numbers.push(num);
                }
            }

            if !numbers.is_empty() {
                problems.push(GridCol {
                    numbers,
                    operation: op,
                });
            }
        }
    }

    let total: u64 = problems.iter().map(|col| col.solve()).sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
