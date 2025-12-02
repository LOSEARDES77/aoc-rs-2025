advent_of_code::solution!(2);

struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    fn parse(raw_range: &str) -> Self {
        let (start, end) = raw_range
            .trim()
            .split_once("-")
            .unwrap_or_else(|| panic!("Error parsing range {}", raw_range));

        let (start, end) = (
            start
                .parse()
                .unwrap_or_else(|e| panic!("Invalid raw_range: {}\nDetails: {}", raw_range, e)),
            end.parse()
                .unwrap_or_else(|e| panic!("Invalid raw_range: {}\nDetails: {}", raw_range, e)),
        );

        IdRange { start, end }
    }

    fn get_ids_in_range(&self) -> Vec<u64> {
        (self.start..=self.end).collect()
    }
}

fn is_invalid_id(id: u64) -> bool {
    let id_string = id.to_string();
    let (left, right) = id_string.split_at(id_string.len() / 2);

    left == right
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_ids = 0;

    for raw_range in input.split(",") {
        let range = IdRange::parse(raw_range);

        for id in range.get_ids_in_range() {
            if is_invalid_id(id) {
                invalid_ids += id
            }
        }
    }

    Some(invalid_ids)
}

fn is_invalid_id_p2(id: u64) -> bool {
    // Use prefix-function (KMP) to detect if the string is a repetition of a smaller substring.
    // Reference: https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm
    let s = id.to_string();
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n <= 1 {
        return false;
    }

    let mut pi = vec![0usize; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && bytes[i] != bytes[j] {
            j = pi[j - 1];
        }
        if bytes[i] == bytes[j] {
            j += 1;
        }
        pi[i] = j;
    }

    let l = pi[n - 1];
    if l == 0 {
        return false;
    }
    let p = n - l;
    n.is_multiple_of(p)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_ids = 0;

    for raw_range in input.split(",") {
        let range = IdRange::parse(raw_range);

        for id in range.get_ids_in_range() {
            if is_invalid_id_p2(id) {
                invalid_ids += id
            }
        }
    }

    Some(invalid_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
