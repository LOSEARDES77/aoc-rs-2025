advent_of_code::solution!(7);

use std::collections::HashMap;

#[derive(Debug)]
struct Grid {
    src_pos: (usize, usize),
    splitters: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().lines().collect();
        let height = lines.len();
        let width = lines[0].len();

        let mut src_pos = (0, 0);
        let mut splitters = Vec::new();

        // Find the starting position 'S' and all splitters '^'
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    'S' => src_pos = (row, col),
                    '^' => splitters.push((row, col)),
                    _ => {}
                }
            }
        }

        Grid {
            src_pos,
            splitters,
            width,
            height,
        }
    }

    // Part 1: Count the number of splits
    fn count_splits(&self) -> u64 {
        let mut split_count = 0;
        let mut current_positions = vec![self.src_pos.1]; // Start with column of S

        // Process each row from the start position downward
        for row in (self.src_pos.0 + 1)..self.height {
            let mut next_positions = Vec::new();

            for &col in &current_positions {
                // Check if there's a splitter at this position
                if self.splitters.contains(&(row, col)) {
                    split_count += 1;
                    // Add left and right positions if they're valid
                    if col > 0 {
                        next_positions.push(col - 1);
                    }
                    if col < self.width - 1 {
                        next_positions.push(col + 1);
                    }
                } else {
                    // No splitter, beam continues straight down
                    next_positions.push(col);
                }
            }

            // Remove duplicates and sort for consistency
            next_positions.sort_unstable();
            next_positions.dedup();
            current_positions = next_positions;
        }

        split_count
    }

    // Part 2: Count the number of unique paths (timelines)
    fn count_paths(&self) -> u64 {
        // Use memoization to avoid recalculating the same state
        let mut memo: HashMap<(usize, usize), u64> = HashMap::new();
        self.count_paths_recursive(self.src_pos.0, self.src_pos.1, &mut memo)
    }

    fn count_paths_recursive(
        &self,
        row: usize,
        col: usize,
        memo: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        // Base case: if we're at or past the last row, this is one complete path
        if row >= self.height {
            return 1;
        }

        // Check if we've already computed this state
        if let Some(&cached) = memo.get(&(row, col)) {
            return cached;
        }

        let mut paths = 0;

        // Check if there's a splitter at the next row at our current column
        let next_row = row + 1;
        if next_row < self.height && self.splitters.contains(&(next_row, col)) {
            // There's a splitter, so we split into left and right paths
            if col > 0 {
                paths += self.count_paths_recursive(next_row, col - 1, memo);
            }
            if col < self.width - 1 {
                paths += self.count_paths_recursive(next_row, col + 1, memo);
            }
        } else {
            // No splitter, continue straight down
            paths += self.count_paths_recursive(next_row, col, memo);
        }

        // Cache the result
        memo.insert((row, col), paths);
        paths
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::parse(input);
    Some(grid.count_splits())
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::parse(input);
    Some(grid.count_paths())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
