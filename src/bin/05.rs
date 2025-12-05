use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn parse_fresh_ids(raw: &str) -> Vec<RangeInclusive<usize>> {
    let mut res = Vec::new();
    for line in raw.lines() {
        let (start, end) = line
            .split_once("-")
            .unwrap_or_else(|| panic!("Error parsing line: {}", line));
        res.push(
            start
                .parse()
                .unwrap_or_else(|_| panic!("Error parsing number: {}", start))
                ..=end
                    .parse()
                    .unwrap_or_else(|_| panic!("Error parsing number: {}", end)),
        );
    }

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, stock) = input
        .split_once("\n\n")
        .unwrap_or_else(|| panic!("Error parsing input: {}", input.trim()));

    let ranges_vec = parse_fresh_ids(ranges);
    let mut count = 0;
    for line in stock.lines() {
        let n = line
            .parse()
            .unwrap_or_else(|_| panic!("Error parsing stock: {}", line));
        for range in ranges_vec.clone() {
            if range.contains(&n) {
                count += 1;
                break;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = input
        .split_once("\n\n")
        .unwrap_or_else(|| panic!("Error parsing input: {}", input.trim()));
    let ranges_vec = parse_fresh_ids(ranges);

    // Merge overlapping ranges and count total coverage
    let mut sorted_ranges = ranges_vec;
    sorted_ranges.sort_by_key(|r| *r.start());

    let mut total = 0u64;
    let mut current_start = *sorted_ranges[0].start();
    let mut current_end = *sorted_ranges[0].end();

    for range in sorted_ranges.iter().skip(1) {
        let start = *range.start();
        let end = *range.end();

        if start <= current_end + 1 {
            current_end = current_end.max(end);
        } else {
            total += (current_end - current_start + 1) as u64;
            current_start = start;
            current_end = end;
        }
    }

    total += (current_end - current_start + 1) as u64;

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
