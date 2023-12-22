use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::common::{parse_lines, Vec2i, Vec3i};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Brick {
    min: Vec3i,
    max: Vec3i,
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_vec3i(s: &str) -> Result<Vec3i, ()> {
            let (x, y, z) = s.split(',').map(str::trim).collect_tuple().ok_or(())?;
            Ok(Vec3i::new(
                x.parse().map_err(|_| ())?,
                y.parse().map_err(|_| ())?,
                z.parse().map_err(|_| ())?,
            ))
        }

        let (min, max) = s.split_once('~').ok_or(())?;
        Ok(Self {
            min: parse_vec3i(min)?,
            max: parse_vec3i(max)?,
        })
    }
}

impl Brick {
    fn fix_bounds(&mut self) {
        let new_min = Vec3i::new(
            self.min.x.min(self.max.x),
            self.min.y.min(self.max.y),
            self.min.z.min(self.max.z),
        );
        let new_max = Vec3i::new(
            self.min.x.max(self.max.x),
            self.min.y.max(self.max.y),
            self.min.z.max(self.max.z),
        );
        self.min = new_min;
        self.max = new_max;
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = parse_lines(input).unwrap();
    bricks.iter_mut().for_each(|b| b.fix_bounds());
    bricks.sort_by_key(|b| (b.min.z, b.min.y, b.min.x));
    bricks
}

const FLOOR: i64 = 0;

fn simulate_bricks(
    bricks: &[Brick],
) -> (
    FxHashMap<Vec2i, (i64, usize)>,
    FxHashMap<usize, FxHashSet<usize>>,
    FxHashMap<usize, FxHashSet<usize>>,
) {
    let mut heightmap: FxHashMap<Vec2i, (i64, usize)> = FxHashMap::default();
    let mut supporting: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();
    let mut supported_by: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();

    for (i, b) in bricks.iter().enumerate() {
        debug_assert!(b.min.z > FLOOR);

        let mut max_resting_height = FLOOR + 1;
        for y in b.min.y..=b.max.y {
            for x in b.min.x..=b.max.x {
                let resting_height = 1 + heightmap
                    .get(&Vec2i::new(x, y))
                    .map(|(h, _)| *h)
                    .unwrap_or(FLOOR);
                if resting_height > max_resting_height {
                    max_resting_height = resting_height;
                }
            }
        }

        let additional_brick_height = b.max.z - b.min.z;
        for y in b.min.y..=b.max.y {
            for x in b.min.x..=b.max.x {
                let pos = Vec2i::new(x, y);
                if let Some((support_height, support_index)) = heightmap.get(&pos) {
                    if 1 + support_height == max_resting_height {
                        supporting.entry(*support_index).or_default().insert(i);
                        supported_by.entry(i).or_default().insert(*support_index);
                    }
                }

                heightmap.insert(pos, (max_resting_height + additional_brick_height, i));
            }
        }
    }

    (heightmap, supporting, supported_by)
}

#[aoc(day22, part1)]
pub fn part1(bricks: &[Brick]) -> usize {
    let (_heightmap, supporting, supported_by) = simulate_bricks(bricks);

    let mut count = 0;
    for i in 0..bricks.len() {
        let Some(on_top) = supporting.get(&i) else {
            count += 1;
            continue;
        };

        if on_top.iter().all(|top_index| {
            let below = &supported_by[top_index];
            below.iter().any(|below_index| *below_index != i)
        }) {
            count += 1;
            continue;
        }
    }

    count
}

#[aoc(day22, part2)]
pub fn part2(bricks: &[Brick]) -> usize {
    let (_heightmap, supporting, supported_by) = simulate_bricks(bricks);

    let mut count = 0;
    for i in 0..bricks.len() {
        let mut falling = FxHashSet::default();
        let mut q = vec![i];
        while let Some(i) = q.pop() {
            falling.insert(i);
            let Some(on_top) = supporting.get(&i) else {
                continue;
            };

            for top_index in on_top {
                let below = &supported_by[top_index];
                if below
                    .iter()
                    .all(|below_index| falling.contains(below_index))
                {
                    q.push(*top_index);
                }
            }
        }

        count += falling.len() - 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 7);
    }
}
