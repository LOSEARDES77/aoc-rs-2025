advent_of_code::solution!(10);

use std::collections::HashMap;

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
            use std::collections::HashSet;
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

fn solve_ilp(machine: &Machine) -> usize {
    let _n_buttons = machine.buttons.len();
    let n_positions = machine.jolt.len();

    // DFS with memoization
    fn dfs(
        current: &[i32],
        target: &[usize],
        buttons: &[Vec<usize>],
        _button_idx: usize,
        memo: &mut HashMap<Vec<i32>, Option<usize>>,
    ) -> Option<usize> {
        // Check if current state matches target
        if current
            .iter()
            .zip(target.iter())
            .all(|(c, t)| *c == *t as i32)
        {
            return Some(0);
        }

        // Check if any value exceeded target
        if current
            .iter()
            .zip(target.iter())
            .any(|(c, t)| *c > *t as i32)
        {
            return None;
        }

        // Check memo
        if let Some(&result) = memo.get(current) {
            return result;
        }

        // Try all buttons
        let mut best = None;

        for (idx, button) in buttons.iter().enumerate() {
            let mut next = current.to_vec();
            for &pos in button {
                next[pos] += 1;
            }

            if let Some(cost) = dfs(&next, target, buttons, idx, memo) {
                let total = cost + 1;
                best = Some(best.map_or(total, |b: usize| b.min(total)));
            }
        }

        memo.insert(current.to_vec(), best);
        best
    }

    let initial = vec![0i32; n_positions];
    let mut memo = HashMap::new();

    dfs(&initial, &machine.jolt, &machine.buttons, 0, &mut memo).unwrap_or(usize::MAX)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);

    let sum: usize = machines.par_iter().map(solve_ilp).sum();

    Some(sum as u64)
}
