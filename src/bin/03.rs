advent_of_code::solution!(3);

#[derive(Debug)]
struct Battery {
    joltage: u8,
}
#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<Battery>,
}

impl BatteryBank {
    fn parse(batteries: &str) -> Self {
        let mut bank = Vec::new();
        for battery in batteries.chars() {
            bank.push(Battery {
                joltage: battery
                    .to_string()
                    .parse()
                    .unwrap_or_else(|_| panic!("Error parsing digit: \"{}\"", battery)),
            });
        }
        BatteryBank { batteries: bank }
    }

    fn get_max_joltaje(&self, digits_to_take: usize) -> u64 {
        let mut start_pos = 0;
        let mut digits = String::new();
        for _ in 0..digits_to_take {
            let (digit, next_satrt_pos) =
                self.get_bigges_joltaje_bat(start_pos, digits_to_take - digits.len());
            digits.push_str(&digit.to_string());
            start_pos = next_satrt_pos + 1;
        }

        digits.parse().unwrap_or_else(|_| {
            panic!(
                "Error calculating bat joltaje with \"{}\" as joltaje",
                digits
            )
        })
    }

    fn get_bigges_joltaje_bat(&self, start_pos: usize, needed_positions: usize) -> (u8, usize) {
        let (mut max, mut pos) = (0, 0);
        for (i, bat) in self.batteries.iter().enumerate().skip(start_pos) {
            if bat.joltage > max {
                if (self.batteries.len() - i) < needed_positions {
                    break;
                }
                max = bat.joltage;
                pos = i;
            }
        }
        (max, pos)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.trim().lines() {
        if !line.is_empty() {
            let bank = BatteryBank::parse(line.trim());
            let j = bank.get_max_joltaje(2);
            sum += j
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.trim().lines() {
        if !line.is_empty() {
            let bank = BatteryBank::parse(line.trim());
            let j = bank.get_max_joltaje(12);
            sum += j
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
