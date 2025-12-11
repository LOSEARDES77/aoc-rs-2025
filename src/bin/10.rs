advent_of_code::solution!(10);

use std::collections::VecDeque;
use z3::ast::Int;
use z3::{Config, Context, Optimize, SatResult};

#[derive(Debug, Clone)]
struct Machine {
    num_lights: usize,
    lighting_goal: u16,
    button_masks: Vec<u16>,
    button_wires: Vec<Vec<usize>>,
    joltage_goal: Vec<usize>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let (rest, joltage_part) = line.split_once('{').unwrap();
        let (lights_part, buttons_part) = rest.split_once(']').unwrap();
        let lights_str = lights_part.trim_start_matches('[');
        let num_lights = lights_str.len();

        let mut lighting_goal = 0;
        for (i, c) in lights_str.chars().enumerate() {
            if c == '#' {
                lighting_goal |= 1 << (num_lights - i - 1);
            }
        }

        let joltage_goal: Vec<usize> = joltage_part
            .trim_end_matches('}')
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let mut button_masks = Vec::new();
        let mut button_wires = Vec::new();

        for segment in buttons_part.split('(').skip(1) {
            let content = segment.split(')').next().unwrap();
            let wires: Vec<usize> = content
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();

            let mut mask = 0;
            for &wire in &wires {
                mask |= 1 << (num_lights - wire - 1);
            }

            button_wires.push(wires);
            button_masks.push(mask);
        }

        Machine {
            num_lights,
            lighting_goal,
            button_masks,
            button_wires,
            joltage_goal,
        }
    }

    fn min_lighting_presses(&self) -> Option<usize> {
        let target = self.lighting_goal;
        let limit = 1 << self.num_lights;
        let mut visited = vec![false; limit];
        let mut queue = VecDeque::new();

        queue.push_back((0u16, 0usize));
        visited[0] = true;

        while let Some((current, steps)) = queue.pop_front() {
            for &mask in &self.button_masks {
                let next_val = current ^ mask;

                if next_val == target {
                    return Some(steps + 1);
                }

                let next_idx = next_val as usize;
                if next_idx < limit && !visited[next_idx] {
                    visited[next_idx] = true;
                    queue.push_back((next_val, steps + 1));
                }
            }
        }

        None
    }

    fn min_joltage_presses(&self) -> Option<u64> {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let opt = Optimize::new(&ctx);

        let mut press_counts = Vec::new();
        let zero = Int::from_i64(&ctx, 0);
        let mut total_presses_expr = Int::from_i64(&ctx, 0);

        // Create variables for each button
        for i in 0..self.button_wires.len() {
            let x_i = Int::new_const(&ctx, format!("x_{}", i));
            opt.assert(&x_i.ge(&zero));
            total_presses_expr = Int::add(&ctx, &[&total_presses_expr, &x_i]);
            press_counts.push(x_i);
        }

        // Constraint: for each light, sum of button presses must equal joltage goal
        for light_idx in 0..self.num_lights {
            let mut light_sum = Int::from_i64(&ctx, 0);

            for (btn_idx, wires) in self.button_wires.iter().enumerate() {
                if wires.contains(&light_idx) {
                    light_sum = Int::add(&ctx, &[&light_sum, &press_counts[btn_idx]]);
                }
            }

            let goal_val = self.joltage_goal[light_idx] as i64;
            opt.assert(&light_sum._eq(&Int::from_i64(&ctx, goal_val)));
        }

        // Minimize total presses
        opt.minimize(&total_presses_expr);

        match opt.check(&[]) {
            SatResult::Sat => {
                let model = opt.get_model()?;
                let result = model.eval(&total_presses_expr, true)?;
                result.as_u64()
            }
            _ => None,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines: Vec<Machine> = input.lines().map(Machine::parse).collect();

    let sum: usize = machines
        .iter()
        .map(|m| m.min_lighting_presses().unwrap())
        .sum();

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines: Vec<Machine> = input.lines().map(Machine::parse).collect();

    let sum: u64 = machines
        .iter()
        .map(|m| m.min_joltage_presses().unwrap())
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,
4,7}",
        );
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(result, Some(11));
    }
}
