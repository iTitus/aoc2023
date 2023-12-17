use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dijkstra;
use tinyvec::array_vec;

use crate::common::{Direction, Grid, Vec2i};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct HeatLoss(u8);

impl TryFrom<char> for HeatLoss {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value.to_digit(10).map(|n| HeatLoss(n as u8)).ok_or(())
    }
}

fn find_shortest_path(
    grid: &Grid<HeatLoss>,
    start: &Vec2i,
    end: &Vec2i,
    min_straight: u8,
    max_straight: u8,
) -> u32 {
    debug_assert!(min_straight <= max_straight);
    let (_, cost) = dijkstra(
        &(*start, Option::<(Direction, u8)>::None),
        |(pos, straight)| {
            let mut v = array_vec!([((Vec2i, Option<(Direction, u8)>), u32); 3]);
            for dir in Direction::VALUES {
                let straight_amount = match straight {
                    None => 1,
                    Some((straight_dir, straight_amount)) => {
                        if straight_dir.opposite() == dir {
                            continue;
                        }

                        if *straight_dir == dir {
                            if *straight_amount >= max_straight {
                                continue;
                            }

                            straight_amount + 1
                        } else {
                            if *straight_amount < min_straight {
                                continue;
                            }

                            1
                        }
                    }
                };

                let offset_pos = dir.offset(pos);
                if !grid.in_bounds(&offset_pos) {
                    continue;
                }

                v.push((
                    (offset_pos, Some((dir, straight_amount))),
                    grid[offset_pos].0 as _,
                ));
            }
            v
        },
        |(pos, straight)| pos == end && (straight.is_none() || straight.unwrap().1 >= min_straight),
    )
    .unwrap();
    cost
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Grid<HeatLoss> {
    input.parse().unwrap()
}

#[aoc(day17, part1)]
pub fn part1(input: &Grid<HeatLoss>) -> u32 {
    find_shortest_path(
        input,
        &Vec2i::new(0, 0),
        &Vec2i::new((input.size_x - 1) as _, (input.size_y - 1) as _),
        0,
        3,
    )
}

#[aoc(day17, part2)]
pub fn part2(input: &Grid<HeatLoss>) -> u32 {
    find_shortest_path(
        input,
        &Vec2i::new(0, 0),
        &Vec2i::new((input.size_x - 1) as _, (input.size_y - 1) as _),
        4,
        10,
    )
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    const INPUT_2: &str = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 102);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&input_generator(INPUT)), 94);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&input_generator(INPUT_2)), 71);
    }
}
