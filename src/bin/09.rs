advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct RedTile {
    x: isize,
    y: isize,
}

impl RedTile {
    fn parse(line: &str) -> Self {
        let (x, y) = line.split_once(",").unwrap();

        RedTile {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
    fn get_rect(self, other: &Self) -> (usize, usize) {
        (
            (self.x - other.x).unsigned_abs() + 1,
            (self.y - other.y).unsigned_abs() + 1,
        )
    }
}

fn calc_rect_area(a: &RedTile, b: &RedTile) -> usize {
    let (x, y) = a.get_rect(b);
    x * y
}

// Check if rectangle defined by a and b does not intersect with the line segment
fn rect_does_not_intersect_line(
    a: &RedTile,
    b: &RedTile,
    line_start: &RedTile,
    line_end: &RedTile,
) -> bool {
    let rect_min_x = a.x.min(b.x);
    let rect_max_x = a.x.max(b.x);
    let rect_min_y = a.y.min(b.y);
    let rect_max_y = a.y.max(b.y);

    let line_min_x = line_start.x.min(line_end.x);
    let line_max_x = line_start.x.max(line_end.x);
    let line_min_y = line_start.y.min(line_end.y);
    let line_max_y = line_start.y.max(line_end.y);

    // Check if line is completely to the left of rectangle
    let left_of_rect = rect_max_x <= line_min_x;

    // Check if line is completely to the right of rectangle
    let right_of_rect = rect_min_x >= line_max_x;

    // Check if line is completely above rectangle
    let above = rect_max_y <= line_min_y;

    // Check if line is completely below rectangle
    let below = rect_min_y >= line_max_y;

    left_of_rect || right_of_rect || above || below
}

pub fn part_one(input: &str) -> Option<u64> {
    let red_tiles: Vec<RedTile> = input.lines().map(RedTile::parse).collect();

    let mut max = 0;

    for rt in &red_tiles {
        for rt2 in &red_tiles {
            max = max.max(calc_rect_area(rt, rt2))
        }
    }

    Some(max as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let red_tiles: Vec<RedTile> = input.lines().map(RedTile::parse).collect();

    // Build the lines (edges) of the polygon
    let mut lines: Vec<(&RedTile, &RedTile)> = Vec::new();
    for i in 0..red_tiles.len() {
        let j = (i + 1) % red_tiles.len();
        lines.push((&red_tiles[i], &red_tiles[j]));
    }

    let mut max = 0;

    // Try all pairs of red tiles as opposite corners
    for i in 0..red_tiles.len() {
        for j in 0..red_tiles.len() {
            if i == j {
                continue;
            }

            let rt1 = &red_tiles[i];
            let rt2 = &red_tiles[j];

            // Check if the rectangle does not intersect any line of the polygon
            let valid = lines.iter().all(|(line_start, line_end)| {
                rect_does_not_intersect_line(rt1, rt2, line_start, line_end)
            });

            if valid {
                max = max.max(calc_rect_area(rt1, rt2));
            }
        }
    }

    Some(max as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
