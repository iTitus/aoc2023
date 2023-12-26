use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::Integer;
use rustc_hash::FxHashMap;

use crate::common::parse_split;

#[derive(Debug, Clone)]
pub enum ModuleType {
    FlipFlop(bool),
    Conjunction(FxHashMap<String, bool>),
    Broadcast,
}

impl ModuleType {
    fn receive_pulse(&mut self, source: &str, pulse: bool) -> Option<bool> {
        match self {
            Self::FlipFlop(state) => {
                if pulse {
                    None
                } else {
                    let new_state = !*state;
                    *state = new_state;
                    Some(new_state)
                }
            }
            Self::Conjunction(state) => {
                *state.get_mut(source).unwrap() = pulse;
                if state.values().all(|v| *v) {
                    Some(false)
                } else {
                    Some(true)
                }
            }
            Self::Broadcast => Some(pulse),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuleConfiguration {
    module_type: ModuleType,
    outputs: Vec<String>,
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> FxHashMap<String, ModuleConfiguration> {
    let mut modules: FxHashMap<String, ModuleConfiguration> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (name, outputs) = l.split_once("->").ok_or(())?;
            let mut name = name.trim();
            let module_type = match name.chars().next().ok_or(())? {
                '%' => {
                    name = &name[1..];
                    ModuleType::FlipFlop(false)
                }
                '&' => {
                    name = &name[1..];
                    ModuleType::Conjunction(FxHashMap::default())
                }
                _ => ModuleType::Broadcast,
            };
            Ok((
                name.to_string(),
                ModuleConfiguration {
                    module_type,
                    outputs: parse_split(outputs, ',').map_err(|_| ())?,
                },
            ))
        })
        .collect::<Result<_, ()>>()
        .unwrap();

    let conjunctions: Vec<_> = modules
        .iter()
        .filter(|(_, m)| matches!(m.module_type, ModuleType::Conjunction(_)))
        .map(|(name, _)| name.to_string())
        .collect();
    for conjunction in &conjunctions {
        let inputs: Vec<_> = modules
            .iter()
            .filter(|(_, m)| m.outputs.contains(conjunction))
            .map(|(name, _)| name.to_string())
            .collect();
        match &mut modules.get_mut(conjunction).unwrap().module_type {
            ModuleType::Conjunction(state) => {
                for input in inputs {
                    state.insert(input, false);
                }
            }
            _ => unreachable!(),
        }
    }

    modules
}

#[aoc(day20, part1)]
pub fn part1(modules: &FxHashMap<String, ModuleConfiguration>) -> i64 {
    const AMOUNT: i64 = 1000;

    let mut modules = modules.clone();
    let mut low = 0;
    let mut high = 0;
    let mut i = 0;
    let mut q = VecDeque::new();
    loop {
        if i >= AMOUNT {
            break;
        }

        q.push_back(("button".to_string(), "broadcaster".to_string(), false));
        while let Some((source, target, pulse)) = q.pop_front() {
            if pulse {
                high += 1;
            } else {
                low += 1;
            }

            if let Some(m) = modules.get_mut(&target) {
                if let Some(new_pulse) = m.module_type.receive_pulse(&source, pulse) {
                    for out in &m.outputs {
                        q.push_back((target.to_string(), out.to_string(), new_pulse));
                    }
                }
            }
        }

        i += 1;
        if modules.values().all(|m| match &m.module_type {
            ModuleType::FlipFlop(state) => !state,
            ModuleType::Conjunction(state) => state.values().all(|s| !s),
            ModuleType::Broadcast => true,
        }) {
            let cycles = AMOUNT / i;
            low *= cycles;
            high *= cycles;
            i *= cycles;
        }
    }

    low * high
}

#[aoc(day20, part2)]
pub fn part2(modules: &FxHashMap<String, ModuleConfiguration>) -> u64 {
    fn get_button_presses_until(
        mut modules: FxHashMap<String, ModuleConfiguration>,
        expected_source: &str,
        expected_target: &str,
        expected_pulse: bool,
    ) -> u64 {
        let mut i = 0;
        let mut q = VecDeque::new();
        loop {
            i += 1;
            q.push_back(("button".to_string(), "broadcaster".to_string(), false));
            while let Some((source, target, pulse)) = q.pop_front() {
                if pulse == expected_pulse && source == expected_source && target == expected_target
                {
                    return i;
                }

                if let Some(m) = modules.get_mut(&target) {
                    if let Some(new_pulse) = m.module_type.receive_pulse(&source, pulse) {
                        for out in &m.outputs {
                            q.push_back((target.to_string(), out.to_string(), new_pulse));
                        }
                    }
                }
            }
        }
    }

    let input = modules
        .iter()
        .filter(|(_, m)| m.outputs.iter().any(|o| o == "rx"))
        .map(|(name, _)| name.to_string())
        .exactly_one()
        .unwrap();
    let conj_inputs: Vec<_> =
        if let ModuleType::Conjunction(conj_state) = &modules[&input].module_type {
            conj_state.keys().map(|name| name.to_string()).collect()
        } else {
            panic!("assume: {input} is conjunction");
        };
    println!("{conj_inputs:?} -> &{input} -> rx");

    // assume looping inputs
    // assume lots of low pulses and then exactly one high pulse
    let mut result = 1;
    for i in &conj_inputs {
        let n = get_button_presses_until(modules.clone(), i.as_str(), input.as_str(), true);
        println!("{n} button presses until {i} sends high pulse to {input}");
        result = result.lcm(&n);
    }

    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT_1: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    const INPUT_2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator(INPUT_1)), 32000000);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 11687500);
    }
}
