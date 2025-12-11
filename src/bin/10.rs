advent_of_code::solution!(10);

use std::collections::HashSet;

use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Machine {
    goal: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    jolt: Vec<usize>,
}

fn parse_goal(s: &str) -> Vec<bool> {
    s.trim_matches(|c| c == '[' || c == ']')
        .chars()
        .map(|c| c == '#')
        .collect()
}

fn parse_button(s: &str) -> Vec<usize> {
    s.trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_joltage(s: &str) -> Vec<usize> {
    s.trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_machine(line: &str) -> Machine {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let goal = parse_goal(parts[0]);
    let mut buttons = Vec::new();

    let mut i = 1;
    while i < parts.len() && parts[i].starts_with('(') {
        buttons.push(parse_button(parts[i]));
        i += 1;
    }

    let jolt = parse_joltage(parts[i]);

    Machine {
        goal,
        buttons,
        jolt,
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.lines().map(parse_machine).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);

    let sum: usize = machines
        .iter()
        .map(|machine| {
            let mut states = HashSet::new();
            let initial_state = vec![false; machine.goal.len()];
            states.insert(initial_state);

            let mut steps = 0;
            loop {
                if states.contains(&machine.goal) {
                    break;
                }

                let mut next_states = HashSet::new();
                for state in states.iter() {
                    for button in &machine.buttons {
                        let mut new_state = state.clone();
                        for &idx in button {
                            new_state[idx] = !new_state[idx];
                        }
                        next_states.insert(new_state);
                    }
                }

                states = next_states;
                steps += 1;
            }

            steps
        })
        .sum();

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);

    let sum: usize = machines
        .par_iter()
        .map(|machine| {
            let mut states = HashSet::new();
            let initial_state = vec![0; machine.jolt.len()];
            states.insert(initial_state);

            let mut steps = 0;

            loop {
                steps += 1;

                let mut next_states = HashSet::new();
                for state in &states {
                    for button in &machine.buttons {
                        let mut new_state = state.clone();
                        let mut valid = true;

                        for &idx in button {
                            new_state[idx] += 1;
                            if new_state[idx] > machine.jolt[idx] {
                                valid = false;
                                break;
                            }
                        }

                        if valid {
                            if new_state == machine.jolt {
                                return steps;
                            }
                            next_states.insert(new_state);
                        }
                    }
                }

                states = next_states;

                if states.is_empty() {
                    return steps;
                }
            }
        })
        .sum();

    Some(sum as u64)
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
}
