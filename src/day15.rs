use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;
use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split(',').map(|s| s.trim().to_string()).collect()
}

fn hash(s: &str) -> u8 {
    debug_assert!(s.is_ascii());
    let mut n = 0u8;
    s.bytes()
        .for_each(|b| n = n.wrapping_add(b).wrapping_mul(17));
    n
}

#[aoc(day15, part1)]
pub fn part1(input: &[String]) -> usize {
    input.iter().map(|s| hash(s) as usize).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &[String]) -> usize {
    let mut boxes: FxHashMap<u8, Vec<(&str, u8)>> = HashMap::default();
    for s in input {
        if let Some((label, focal_length)) = s.split_once('=') {
            let focal_length = focal_length.parse().unwrap();
            let box_contents = boxes.entry(hash(label)).or_default();
            if let Some((_, ex_focal_length)) = box_contents
                .iter_mut()
                .find(|(ex_label, _)| *ex_label == label)
            {
                *ex_focal_length = focal_length;
            } else {
                box_contents.push((label, focal_length));
            }
        } else if let Some(label) = s.strip_suffix('-') {
            let box_contents = boxes.entry(hash(label)).or_default();
            if let Some(pos) = box_contents
                .iter()
                .position(|(ex_label, _)| *ex_label == label)
            {
                box_contents.remove(pos);
            }
        } else {
            panic!("invalid input");
        }
    }

    boxes
        .iter()
        .flat_map(|(n, box_contents)| {
            let box_id = *n as usize + 1;
            box_contents
                .iter()
                .enumerate()
                .map(move |(i, (_, focal_length))| box_id * (i + 1) * *focal_length as usize)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 145);
    }
}
