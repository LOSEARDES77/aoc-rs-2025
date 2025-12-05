advent_of_code::solution!(4);

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn count_adjacent_papers(grid: &[Vec<bool>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for (dr, dc) in DIRECTIONS.iter() {
        let new_r = row as i32 + dr;
        let new_c = col as i32 + dc;

        if new_r >= 0
            && new_r < rows as i32
            && new_c >= 0
            && new_c < cols as i32
            && grid[new_r as usize][new_c as usize]
        {
            count += 1;
        }
    }

    count
}

fn find_accessible_rolls(grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell {
                let adjacent_papers = count_adjacent_papers(grid, r, c);
                if adjacent_papers < 4 {
                    accessible.push((r, c));
                }
            }
        }
    }

    accessible
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);

    if grid.is_empty() {
        return Some(0);
    }

    let accessible = find_accessible_rolls(&grid);
    Some(accessible.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);

    if grid.is_empty() {
        return Some(0);
    }

    let mut total_removed = 0;

    loop {
        let accessible = find_accessible_rolls(&grid);

        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (r, c) in accessible.iter() {
            grid[*r][*c] = false;
        }

        total_removed += accessible.len();
    }

    Some(total_removed as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
